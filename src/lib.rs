use anyhow::{bail, Result};
use libloading::{Library, Symbol};
use pdb::{FallibleIterator, PDB};
use retour::static_detour;
use std::ffi::CString;
use std::net::TcpStream;
use std::path::Path;
use std::sync::Mutex;
use std::{collections::HashMap, fs::File};
use tracing::{error, info};
use traits::{factorio_path, AsPcstr};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

mod traits;

struct PDBCache {
    pdb: PDB<'static, File>,
    symbol_addresses: HashMap<String, u32>,
    base_address: u64,
}

impl PDBCache {
    fn new(pdb_path: &Path, module_name: &str) -> Result<Self> {
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
    let result = GetModuleHandleA(CString::new(module_name)?.as_pcstr());
    match result {
        Ok(handle) => Ok(handle.0 as u64),
        Err(err) => bail!(err),
    }
}

fn start_stream() {
    let ip = "127.0.0.1:40267";
    let stream = TcpStream::connect(ip).unwrap_or_else(|_| {
        panic!("Could not establish stdout connection to rivets. Port {ip} is busy.")
    });
    tracing_subscriber::fmt::fmt()
        .with_writer(Mutex::new(stream))
        .init();
}

fn inject() -> Result<()> {
    let pdb_path = factorio_path("factorio.pdb")?;
    let pdb_cache = PDBCache::new(&pdb_path, "factorio.exe")?;

    let dll_paths = [
        r"C:/Users/zacha/Documents/factorio/mods/achievement-enabler/target/debug/achievement_enabler.dll",
    ];
    for path in dll_paths {
        let lib = unsafe { Library::new(path)? };
        let mangled_name = unsafe {
            let func: Symbol<extern "C" fn() -> String> = lib.get(b"mangled_name")?;
            func()
        };
        let Some(address) = pdb_cache.get_function_address(&mangled_name) else {
            bail!("Failed to find main address");
        };
        info!("{mangled_name} address: {:#x}", address);

        static_detour! {
            static Hook: unsafe extern "C" fn() -> bool;
        }

        type Signature = unsafe extern "C" fn() -> bool;

        let main_detour = unsafe {
            let func: Symbol<fn() -> Signature> = lib.get(b"main_detour")?;
            func()
        };
        info!("jjjjjjjjjjjjjj {}", unsafe{ main_detour()});

        unsafe {
            let target: Signature = std::mem::transmute(address);
            Hook.initialize(target, move || {info!("jjj");let q =unsafe {main_detour()};info!("{q}");q})?.enable()?;
        }
    }
    Ok(())
}

#[ctor::ctor]
fn ctor() {
    start_stream();

    if let Err(e) = inject() {
        error!("{e}");
    }
}
