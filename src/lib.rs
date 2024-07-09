#![cfg(all(windows))]

use pdb::{FallibleIterator, PDB};
use retour::static_detour;
use std::error::Error;
use std::ffi::c_char;
use std::iter;
use std::net::TcpStream;
use std::sync::Mutex;
use std::{collections::HashMap, ffi::c_int, fs::File, mem};
use tracing::info;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};

struct PDBCache {
    pdb: PDB<'static, File>,
    symbol_addresses: HashMap<String, u32>,
}

impl PDBCache {
    fn new(pdb_path: &str) -> Result<Self, pdb::Error> {
        let file = File::open(pdb_path)?;
        let pdb = PDB::open(file)?;

        let mut cache = PDBCache {
            pdb,
            symbol_addresses: HashMap::new(),
        };

        cache.build_symbol_map()?;

        Ok(cache)
    }

    fn build_symbol_map(&mut self) -> Result<(), pdb::Error> {
        let symbol_table = self.pdb.global_symbols()?;
        let address_map = self.pdb.address_map()?;

        symbol_table.iter().for_each(|symbol| {
            match symbol.parse() {
                Ok(pdb::SymbolData::Public(data)) if data.function => {
                    let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                    self.symbol_addresses
                        .insert(data.name.to_string().into(), rva.0);
                }
                Err(e) => return Err(e),
                _ => {}
            };

            Ok(())
        })?;

        Ok(())
    }

    pub fn get_function_address(&self, function_name: &str) -> Option<u64> {
        self.symbol_addresses
            .get(function_name)
            .copied()
            .map(|x| x as u64)
    }
}

static_detour! {
    static MainHook: unsafe extern "C" fn(c_int, *const c_char, *const c_char) -> c_int;
}

type FnMain = unsafe extern "C" fn(c_int, *const c_char, *const c_char) -> c_int;

fn main_detour(_argc: c_int, _argv: *const c_char, _envp: *const c_char) -> c_int {
    info!("Detoured into main!");
    //unsafe { MessageBoxWHook.call(hwnd, text, replaced_caption, msgbox_style) }
    0.into()
}

trait AsPcwstr {
    fn as_pcwstr(&self) -> PCWSTR;
}

impl AsPcwstr for str {
    fn as_pcwstr(&self) -> PCWSTR {
        let utf16: Vec<u16> = self.encode_utf16().chain(iter::once(0)).collect();
        PCWSTR(utf16.as_ptr() as _)
    }
}

trait AsPcstr {
    unsafe fn as_pcstr(&self) -> PCSTR;
}

impl AsPcstr for str {
    /// This is unsafe because `PCSTR` does not implement the `Sized` trait and thus its bytes may be overwritten.
    /// Use the `s!` macro to create a safe `PCSTR` at compile-time.
    unsafe fn as_pcstr(&self) -> PCSTR {
        PCSTR(format!("{}\0", self).as_ptr() as _)
    }
}

/// Returns a module symbol's absolute address.
fn get_module_symbol_address(module: &str, symbol: &str) -> Result<usize, String> {
    let handle = unsafe { GetModuleHandleW(module.as_pcwstr()) };
    let handle = handle.map_err(|e| format!("Failed to get module handle: {}", e))?;

    match unsafe { GetProcAddress(handle, symbol.as_pcstr()) } {
        Some(func) => Ok(func as usize),
        None => Err(format!("Failed to find function address: {:?}", unsafe {
            GetLastError()
        })),
    }
}

unsafe fn hook(pdb_cache: PDBCache) -> Result<(), Box<dyn Error>> {
    //let address = get_module_symbol_address("user32.dll", "MessageBoxW")?;
    //info!("main address: {:#x}", address);
    //let target: FnMain = mem::transmute(address);
    info!("q");
    MainHook.initialize(main, main_detour)?.enable()?;
    info!("r");
    Ok(())
}

#[ctor::ctor]
fn ctor() {
    let stream = TcpStream::connect("127.0.55.1:16337").unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    let cache = PDBCache::new(r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.pdb").unwrap();

    let result = unsafe { hook(cache) };
    match result {
        Ok(_) => {}
        Err(e) => {
            info!("{:?}", e);
        }
    }
}
