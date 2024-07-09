#![cfg(all(windows))]
#[allow(unreachable_code, unused_variables, unused_imports)]

use pdb::{FallibleIterator, PDB};
use retour::static_detour;
use tracing::info;
use std::error::Error;
use std::net::TcpStream;
use std::sync::Mutex;
use std::{
    collections::HashMap,
    ffi::c_int,
    fs::File,
    mem,
};

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
    static MainHook: unsafe extern "system" fn(c_int, u32, u32) -> c_int;
}

type FnMain = unsafe extern "system" fn(c_int, u32, u32) -> c_int;

fn main_detour(_argc: c_int, _argv: u32, _envp: u32) -> c_int {
    info!("Detoured into main!");
    //unsafe { MessageBoxWHook.call(hwnd, text, replaced_caption, msgbox_style) }
    0.into()
}

unsafe fn main(pdb_cache: PDBCache) -> Result<(), Box<dyn Error>> {
    let address = match pdb_cache.get_function_address("?save@Recipe@@QEBAXAEAVMapSerialiser@@@Z") {
        Some(address) => address,
        None => {
            return Err("Failed to find main function address".into());
        }
    };
    info!("main address: {:#x}", address);
    let target: FnMain = mem::transmute(address);
    info!("q");

    MainHook.initialize(target, main_detour)?.enable()?;
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

    let result = unsafe { main(cache) };
    match result {
        Ok(_) => {}
        Err(e) => {
            info!("{:?}", e);
        }
    }
}
