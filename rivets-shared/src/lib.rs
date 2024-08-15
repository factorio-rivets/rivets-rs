use anyhow::{bail, Result};
use cpp_demangle::Symbol;
use pdb::{FallibleIterator, PDB};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::path::Path;
use undname::Flags;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

pub trait AsPcstr {
    fn as_pcstr(&self) -> PCSTR;
}

impl AsPcstr for CStr {
    fn as_pcstr(&self) -> PCSTR {
        PCSTR(self.as_ptr().cast())
    }
}

/// Attempts to demangle a mangled MSVC C++ symbol name. First tries MSVC demangling, then falls back to Itanium.
#[must_use]
pub fn demangle(mangled: &str) -> Option<String> {
    undname::demangle(mangled.into(), Flags::NO_ACCESS_SPECIFIER).map_or_else(
        |_| Symbol::new(mangled).ok().map(|x| x.to_string()),
        |x| Some(x.to_string()),
    )
}

/// Takes an unmangled C++ MSVC symbol name and returns the calling convention.
/// Fails if the calling convention is not one of cdecl, stdcall, fastcall, thiscall, or vectorcall.
#[must_use]
pub fn get_calling_convention(abi: &str) -> Option<syn::Abi> {
    Some(match abi {
        "__cdecl" => syn::parse_quote! { extern "C" },
        "__stdcall" => syn::parse_quote! { extern "stdcall" },
        "__fastcall" => syn::parse_quote! { extern "fastcall" },
        "__thiscall" => syn::parse_quote! { extern "thiscall" },
        "__vectorcall" => syn::parse_quote! { extern "vectorcall" },
        _ => return None,
    })
}

/// Repersents a pointer to any opaque FFI data. (normally detour args or FFI struct pointers)
///
/// # Examples
/// ```
/// #[detour(...)]
/// fn run(
///    this: Opaque,
///    lua_event_type: i32,
///    map_tick_type: Opaque,
///    lua_game_script: Opaque,
///    game_action: Opaque,
/// ) {
///     unsafe { back(this, lua_event_type, map_tick_type, lua_game_script, game_action) }
/// }
/// ```
pub type Opaque = *const std::ffi::c_void;

pub struct RivetsHook {
    pub mangled_name: String,
    pub hook: unsafe fn(u64) -> Result<(), retour::Error>,
}

#[derive(SerdeSerialize, SerdeDeserialize)]
pub struct SymbolCache {
    symbol_addresses: HashMap<String, u32>,
    module_name: String,
}

impl SymbolCache {
    /// Creates a new `SymbolCache` instance.
    ///
    /// # Arguments
    /// * `pdb_path` - The path to the PDB file.
    /// * `module_name` - The name of the module to get the base address of.
    #[cfg(target_os = "windows")]
    pub fn new(pdb_path: impl AsRef<Path>, module_name: impl AsRef<str>) -> Result<Self> {
        use anyhow::Context;

        let file = File::open(&pdb_path).with_context(|| format!("Failed to open factorio.pdb file at path {}. Does this file actually exist? If not, reinstall factorio.", pdb_path.as_ref().display()))?;
        let mut pdb = PDB::open(file)?;

        let mut symbol_addresses = HashMap::new();
        let symbol_table = pdb.global_symbols()?;
        let address_map = pdb.address_map()?;

        symbol_table
            .iter()
            .for_each(|symbol| match symbol.parse() {
                Ok(pdb::SymbolData::Public(data)) if data.function => {
                    let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                    symbol_addresses.insert(data.name.to_string().into(), rva.0);
                    Ok(())
                }
                Err(e) => Err(e),
                _ => Ok(()),
            })?;

        Ok(Self {
            symbol_addresses,
            module_name: module_name.as_ref().to_string(),
        })
    }

    #[cfg(target_os = "linux")]
    pub fn new(pdb_path: impl AsRef<Path>, module_name: &str) -> Result<Self> {
        todo!();
    }

    pub fn get_function_address(&self, base_address: u64, mangled_name: &str) -> Option<u64> {
        self.symbol_addresses
            .get(mangled_name)
            .copied()
            .map(|x| base_address + u64::from(x))
    }

    #[cfg(target_os = "windows")]
    pub fn get_module_base_address(&self) -> Result<u64> {
        let module_name = self.module_name.clone();
        let result = unsafe { GetModuleHandleA(CString::new(module_name)?.as_pcstr()) };
        match result {
            Ok(handle) => Ok(handle.0 as u64),
            Err(err) => bail!(err),
        }
    }

    #[cfg(target_os = "linux")]
    fn get_module_base_address(module_name: &str) -> Result<u64> {
        todo!();
    }

    /// Injects a detour into a Factorio compiled function.
    ///
    /// # Arguments
    /// * `factorio_path` - The path to the Factorio binary directory.
    /// * `function_name` - The name of the function to inject the detour into.
    /// * `hook` - The detour function to inject.
    ///
    /// # Safety
    /// todo!
    pub unsafe fn inject(&self, base_address: u64, hook: &RivetsHook) -> Result<()> {
        let Some(address) = self.get_function_address(base_address, hook.mangled_name.as_str()) else {
            bail!(
                "Failed to find address for the following mangled function inside the PDB: {}",
                hook.mangled_name
            );
        };

        Ok((hook.hook)(address)?)
    }
}
