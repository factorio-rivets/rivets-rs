use anyhow::{bail, Result};
use pdb::{FallibleIterator, PDB};
use retour::static_detour;
use std::ffi::c_char;
use std::net::TcpStream;
use std::sync::Mutex;
use std::{collections::HashMap, ffi::c_int, fs::File, mem};
use tracing::{error, info};
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

struct PDBCache {
    pdb: PDB<'static, File>,
    symbol_addresses: HashMap<String, u32>,
    base_address: u64,
}

impl PDBCache {
    fn new(pdb_path: &str, module_name: &str) -> Result<Self> {
        let file = File::open(pdb_path)?;
        let pdb = PDB::open(file)?;
        let base_address = unsafe { get_dll_base_address(module_name)? };

        let mut cache = Self {
            pdb,
            symbol_addresses: HashMap::new(),
            base_address,
        };

        cache.build_symbol_map()?;

        Ok(cache)
    }

    fn build_symbol_map(&mut self) -> Result<()> {
        let symbol_table = self.pdb.global_symbols()?;
        let address_map = self.pdb.address_map()?;

        symbol_table
            .iter()
            .for_each(|symbol| match symbol.parse() {
                Ok(pdb::SymbolData::Public(data)) if data.function => {
                    let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                    self.symbol_addresses
                        .insert(data.name.to_string().into(), rva.0);
                    Ok(())
                }
                Err(e) => Err(e),
                _ => Ok(()),
            })?;

        Ok(())
    }

    pub fn get_function_address(&self, function_name: &str) -> Option<u64> {
        self.symbol_addresses
            .get(function_name)
            .copied()
            .map(|x| self.base_address + u64::from(x))
    }
}

unsafe fn get_dll_base_address(module_name: &str) -> Result<u64> {
    let result = GetModuleHandleA(module_name.as_pcstr());
    match result {
        Ok(handle) => Ok(handle.0 as u64),
        Err(err) => bail!(err),
    }
}

static_detour! {
    static MainHook: unsafe extern "C" fn(c_int, *const c_char, *const c_char) -> bool;
}

type FnMain = unsafe extern "C" fn(c_int, *const c_char, *const c_char) -> bool;

fn main_detour(_argc: c_int, _argv: *const c_char, _envp: *const c_char) -> bool {
    info!("Detoured into main!");
    //unsafe { MessageBoxWHook.call(hwnd, text, replaced_caption, msgbox_style) }
    false
}

trait AsPcstr {
    unsafe fn as_pcstr(&self) -> PCSTR;
}

impl AsPcstr for str {
    /// This is unsafe because `PCSTR` does not implement the `Sized` trait and thus its bytes may be overwritten.
    /// Use the `s!` macro to create a safe `PCSTR` at compile-time.
    unsafe fn as_pcstr(&self) -> PCSTR {
        PCSTR(format!("{self}\0").as_ptr().cast())
    }
}

unsafe fn hook(pdb_cache: &PDBCache) -> Result<()> {
    let function_name = "?valid@LuaSurface@@UEBA_NXZ";

    let Some(address) = pdb_cache.get_function_address(function_name) else {
        bail!("Failed to find main address");
    };

    info!("{} address: {:#x}", function_name, address);
    let target: FnMain = mem::transmute(address);
    MainHook.initialize(target, main_detour)?.enable()?;
    Ok(())
}

#[ctor::ctor]
fn ctor() {
    let ip = "127.0.0.1:40267";
    let stream = TcpStream::connect(ip).unwrap_or_else(|_| {
        panic!("Could not establish stdout connection to rivets. Port {ip} is busy.")
    });
    tracing_subscriber::fmt::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    let cache = match PDBCache::new(
        r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.pdb",
        "factorio.exe",
    ) {
        Ok(cache) => cache,
        Err(e) => {
            error!("Failed to parse Factorio PDB file. {e:?}");
            return;
        }
    };

    let result = unsafe { hook(&cache) };
    if let Err(e) = result {
        error!("{e:?}");
    }
}
