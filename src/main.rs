use anyhow::{anyhow, Result};
use rivets_shared::factorio_path;
mod pdb2hpp;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let pdb_path = factorio_path("factorio.pdb")?;
    let target = args
        .next()
        .ok_or_else(|| anyhow!("Usage: rivets bindings <decompilation target>"))?;
    pdb2hpp::generate(&pdb_path, &target)?;
    Ok(())
}
