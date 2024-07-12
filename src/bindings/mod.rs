use anyhow::Result;
use pdb2hpp::write_class;

mod pdb2hpp;

pub fn generate(pdb_path: &str) -> Result<()> {
    let class_name = "lua_Debug";

    let header = write_class(pdb_path, class_name)?;

    println!("{header}");

    let bindings = bindgen::Builder::default()
        .header_contents("bindings.hpp", &header)
        .rust_target(bindgen::RustTarget::Nightly)
        .generate()?;

    bindings.write_to_file("./src/structs.rs")?;

    Ok(())
}
