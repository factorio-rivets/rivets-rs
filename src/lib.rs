#![cfg(all(windows))]

use ctor::ctor;
use pdb::{Error, FallibleIterator, PDB};
use std::{collections::HashMap, fs::File, net::TcpStream, sync::Mutex};
use tracing::info;

struct PDBCache {
    pdb: PDB<'static, File>,
    symbol_addresses: HashMap<String, u32>,
}

impl PDBCache {
    fn new(pdb_path: &str) -> Result<Self, Error> {
        let file = File::open(pdb_path)?;
        let pdb = PDB::open(file)?;

        let mut cache = PDBCache {
            pdb,
            symbol_addresses: HashMap::new(),
        };

        cache.build_symbol_map()?;

        Ok(cache)
    }

    fn build_symbol_map(&mut self) -> Result<(), Error> {
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

    pub fn get_function_address(&self, function_name: &str) -> Option<u32> {
        self.symbol_addresses.get(function_name).copied()
    }
}

#[ctor]
fn ctor() {
    let stream = TcpStream::connect("127.0.55.1:16337").unwrap();
    tracing_subscriber::fmt::fmt()
        .with_writer(Mutex::new(stream))
        .init();
    for _ in 0..10 {
        info!("Hi!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }

    let cache = PDBCache::new(r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.pdb").unwrap();
    info!(
        "{}",
        cache
            .get_function_address("?getRemainingDistance@Train@@QEAANXZ")
            .unwrap()
    );
}
