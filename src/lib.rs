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
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

struct PDBCache {
    pdb: PDB<'static, File>,
    symbol_addresses: HashMap<String, u32>,
    base_address: u64,
}

impl PDBCache {
    fn new(pdb_path: &str, module_name: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(pdb_path)?;
        let pdb = PDB::open(file)?;
        let base_address = unsafe { get_dll_base_address(module_name)? };

        let mut cache = PDBCache {
            pdb,
            symbol_addresses: HashMap::new(),
            base_address,
        };

        cache.build_symbol_map()?;

        Ok(cache)
    }

    fn build_symbol_map(&mut self) -> Result<(), Box<dyn Error>> {
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
            .map(|x| self.base_address + x as u64)
    }
}

unsafe fn get_dll_base_address(module_name: &str) -> Result<u64, Box<dyn Error>> {
    let result = GetModuleHandleA(module_name.as_pcstr());
    match result {
        Ok(handle) => Ok(handle.0 as u64),
        Err(err) => Err(err.into()),
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

unsafe fn hook(pdb_cache: PDBCache) -> Result<(), Box<dyn Error>> {
    let address = match pdb_cache.get_function_address("?gameUpdateLoop@MainLoop@@YA?AV?$duration@_JU?$ratio@$00$0DLJKMKAA@@std@@@chrono@std@@W4HeavyMode@1@@Z") {
        Some(address) => address,
        None => return Err("Failed to find main address".into()),
    };

    info!("main address: {:#x}", address);
    let target: FnMain = mem::transmute(address);
    MainHook.initialize(target, main_detour)?.enable()?;
    Ok(())
}

#[ctor::ctor]
fn ctor() {
    let ip = "127.0.0.1:40267";
    let stream = TcpStream::connect(ip).expect(&format!(
        "Could not establish stdout connection to rivets. Port {} is busy.",
        ip
    ));
    tracing_subscriber::fmt::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    let cache = match PDBCache::new(
        r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.pdb",
        "factorio.exe",
    ) {
        Ok(cache) => cache,
        Err(e) => {
            info!("Failed to parse Factorio PDB file. {:?}", e);
            return;
        }
    };

    let result = unsafe { hook(cache) };
    match result {
        Ok(_) => {}
        Err(e) => {
            info!("{:?}", e);
        }
    }
}
