use cpp_demangle::Symbol;
use undname::Flags;
use abi_stable::std_types::{RBoxError, RResult, RString, RVec};
use abi_stable::StableAbi;

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

#[repr(C)]
#[derive(StableAbi)]
pub struct RivetsHook {
    pub mangled_name: RString,
    pub hook: unsafe extern "C" fn(u64) -> RResult<(), RBoxError>,
}

#[repr(C)]
#[derive(StableAbi)]
pub struct RivetsFinalizeABI {
    pub get_hooks: extern "C" fn() -> RVec<RivetsHook>,
}
