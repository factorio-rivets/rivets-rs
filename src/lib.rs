#![cfg(all(windows))]

use std::{fs::File, net::TcpStream, sync::Mutex};
use tracing::info;
use ctor::ctor;

fn x() -> pdb::Result<()> {
    let pdb_path = r"C:\Users\zacha\Documents\factorio\bin\x64\factorio.pdb";
    let file = File::open(pdb_path)?;
    let mut pdb = pdb::PDB::open(file)?;
    let symbol_table = pdb.global_symbols()?;
    let address_map = pdb.address_map()?;

    let mut symbols = symbol_table.iter();
    let _ = symbols.for_each(|symbol| match symbol.parse() {
        Ok(pdb::SymbolData::Public(data)) if data.function => Ok({
            // we found the location of a function!
            let rva = data.offset.to_rva(&address_map).unwrap_or_default();
            println!("{} is {}", rva, data.name);
        }),
        _ => Ok(()),
    });

    Ok(())
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
    
    x();
}
