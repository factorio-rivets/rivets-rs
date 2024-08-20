#![warn(missing_docs)]
#![feature(proc_macro_diagnostic)]

//! Contains the proc macros for rivets.

use anyhow::{bail, Result};
use darling::FromDeriveInput;
use lazy_regex::regex;
use proc_macro::{self, Diagnostic, Level, Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::sync::{atomic::AtomicBool, LazyLock, Mutex};
use syn::{parse_macro_input, Abi, DeriveInput, Error, Expr, FnArg, Ident, ItemFn, Variant};

static IS_FINALIZED: AtomicBool = AtomicBool::new(false);
static MANGLED_NAMES: LazyLock<Mutex<Vec<(String, String)>>> = LazyLock::new(|| Mutex::new(vec![]));
static CPP_IMPORTS: LazyLock<Mutex<Vec<(String, String)>>> = LazyLock::new(|| Mutex::new(vec![]));

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(proc_macro2::Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

macro_rules! check_finalized {
    () => {
        // this check causes issues with rust-analyer. disable during debug builds.
        #[cfg(not(debug_assertions))]
        if IS_FINALIZED.load(std::sync::atomic::Ordering::Relaxed) {
            panic!("The rivets library has already been finalized!");
        }
    };
}

fn failure(callback: proc_macro2::TokenStream, error_message: &str) -> TokenStream {
    Diagnostic::spanned(Span::call_site(), Level::Error, error_message).emit();
    callback.into()
}

fn determine_calling_convention(input: &ItemFn, unmangled_name: &str) -> Result<Abi> {
    if let Some(abi) = &input.sig.abi {
        return Ok(abi.clone());
    }

    let abi = regex!(r" __[a-zA-Z]+ ").find(unmangled_name);
    let abi = match abi {
        Some(abi) => abi.as_str(),
        None => bail!("Failed to automatically determine calling convention for {unmangled_name}. Try specifying the calling convention manually. Example: extern \"C\" fn() {}", "{}"),
    };
    let abi = &abi[1..abi.len() - 1];
    if let Some(calling_convention) = rivets_shared::get_calling_convention(abi) {
        Ok(calling_convention)
    } else {
        bail!("Calling convention {abi} is not currently supported by rivets. Please report this issue to the rivets developers.");
    }
}

/// A procedural macro for detouring a C++ compiled function.
///
/// The argument to the macro is the mangled name of the C++ function to detour.
/// I recommend finding these mangled names by using `IDA Free with the Hex-Rays Decompiler` to inspect the factorio binary.
///
/// This macro is useful in the following scenarios:
///     - Running rust code after a C++ function is called.
///     - Running rust code before a C++ function is called.
///     - Overwriting a C++ function with a rust function.
///     - Preforming some operation on the arguments of a C++ function before calling the original function.
///     - Preforming some operation on the return value of a C++ function before returning it to the caller.
///
/// This macro cannot hook into the middle of a C++ function. It can only hook into the beginning or end of a function.
///
/// This macro cannot hook into a function that has been inlined by the compiler. Prominent examples of this include `lua_gettop`.
///
/// Exposes an `unsafe` `back` function that can be called in order to resume control flow to the original C++ function.
///
/// Internally uses the `retour` crate to create a static detour for the function and thus inherits the safety guarantees of that crate.
///
/// # Examples
/// ```
/// #[detour(?run@LuaEventDispatcher@@AEAAXW4LuaEventType@@VMapTickType@@P8LuaGameScript@@EAA_NAEBVGameAction@@@Z2@Z)]
/// fn run(
///    this: Opaque,
///    lua_event_type: i32,
///    map_tick_type: Opaque,
///    lua_game_script: Opaque,
///    game_action: Opaque,
/// ) {
///     let event: Result<defines::events, _> = (lua_event_type as u8).try_into();
///     if let Ok(event) = event {
///         info!("A simple Rust event handler! {:?}", q);
///     }
///     unsafe { back(this, lua_event_type, map_tick_type, lua_game_script, game_action) }
/// }
/// ```
///
/// # Safety
/// The arguments to the detoured function are the raw FFI pointers to the arguments of the original C++ function.
/// It is up to the user to ensure that the arguments are valid FFI types.
/// All structs, classes, enums, and union arguments must have a corresponding `#[repr(C)]` attribute and must also have the correct offsets and sizes.
/// Alternatively, the user can use the `rivets::Opaque` type to represent any arbitrary FFI data if you do not intend to interact with the data.
/// See the `pdb2hpp` module for a tool that can generate the correct FFI types for C++ functions.
#[proc_macro_attribute]
pub fn detour(attr: TokenStream, item: TokenStream) -> TokenStream {
    check_finalized!();

    let mangled_name = attr.to_string();
    let unmangled_name =
        rivets_shared::demangle(&mangled_name).unwrap_or_else(|| mangled_name.clone());

    let mut input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let return_type = &input.sig.output;

    let inputs = &input.sig.inputs;
    let mut arg_names: Vec<proc_macro2::TokenStream> = vec![];
    let arguments: Vec<proc_macro2::TokenStream> = inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => {
                quote! {compile_error!("Detour functions cannot use the self parameter.")}
            }
            FnArg::Typed(pat) => {
                let attrs = &pat.attrs;
                let ty = &pat.ty;
                let pat = &pat.pat;
                arg_names.push(quote! { #pat });
                quote! { #(#attrs)* #ty }
            }
        })
        .collect();
    let arguments = quote! { #( #arguments ),* };
    let arg_names = quote! { #( #arg_names ),* };

    let calling_convention = match determine_calling_convention(&input, &unmangled_name) {
        Ok(calling_convention) => calling_convention,
        Err(e) => return failure(quote! { #input }, &e.to_string()),
    };
    input.sig.abi = None;
    let callback = quote! { #input };

    let cpp_function_header = quote! {
        unsafe #calling_convention fn(#arguments) #return_type
    };

    let result = quote! {
        mod #name {
            use super::*;

            rivets::retour::static_detour! {
                static Detour : #cpp_function_header;
            }

            unsafe fn back(#inputs) #return_type {
                Detour.call(#arg_names)
            }

            #[doc = #unmangled_name]
            #[allow(unused_variables)]
            #callback

            pub unsafe fn hook(address: u64) -> Result<(), rivets::retour::Error> {
                let compiled_function: #cpp_function_header = std::mem::transmute(address); // todo: rust documentation recommends casting this to a raw function pointer. address as *const _
                Detour.initialize(compiled_function, #name)?.enable()?;
                Ok(())
            }
        }
    };

    MANGLED_NAMES
        .lock()
        .expect("Failed to lock mangled names")
        .push((mangled_name.clone(), name.to_string()));

    Diagnostic::spanned(Span::call_site(), Level::Note, unmangled_name.clone()).emit();

    result.into()
}

/// A procedural macro for importing a C++ compiled function into the rust scope.
/// This macro is useful in the case where you need to directly call any C++ function from rust.
///
/// # Arguments
/// * `mangled_name` - The mangled name of the C++ function to import.
/// * `dll_name` (optional) - Argument for the name of the DLL to import the function from. If not provided, factorio.exe will be used.
///
/// Note that most Factorio libraries (such as allegro and lua) are statically linked. In this case, the `dll_name` argument is not needed.
///
/// # Examples
/// ```
/// // Summons the lua_gettop function from the compiled lua library.
/// // lua_gettop is compiled without name mangling, so calling convention (in this case, extern "C") must be manually provided.
/// #[import(lua_gettop)]
/// extern "C" fn lua_gettop(lua_state: *mut luastate::lua_State) -> i64 {}
///
/// // Calls the lua_gettop function with correct arguments.
/// fn my_func(*mut luastate::lua_State) {
///    let top = unsafe { lua_gettop(lua_state) };
///    println!("Lua stack top: {top}");
/// }
/// ```
///
/// # Safety
/// The arguments and return type of the imported function must be exactly matching FFI types.
/// All structs, classes, enums, and union arguments must have a corresponding `#[repr(C)]` attribute and must also have the correct offsets and sizes.
/// Alternatively, the user can use the `rivets::Opaque` type to represent any arbitrary FFI data if you do not intend to interact with the data.
/// See the `pdb2hpp` module for a tool that can generate the correct FFI types for C++ functions.
///
/// The user must also ensure that the calling convention is correct.
/// Rivets attempts to automatically parse this information from the mangled name however
///     - If the calling convention is not one of cdecl, stdcall, fastcall, thiscall, or vectorcall, the user must specify the calling convention manually.
///     - If the calling convention is not present in the mangled name, the user must specify the calling convention manually.
///     - In rare cases the function may use a non-standard calling convention. In this case, the user must manually populate the required stack and registers via inline assembly.
///
/// Calling any imported function repersents calling into the C++ compiled codebase and thus is inherently unsafe.
#[proc_macro_attribute]
pub fn import(attr: TokenStream, item: TokenStream) -> TokenStream {
    check_finalized!();

    let mangled_name = attr.to_string();
    let unmangled_name =
        rivets_shared::demangle(&mangled_name).unwrap_or_else(|| mangled_name.clone());

    let input = parse_macro_input!(item as ItemFn);

    let calling_convention = match determine_calling_convention(&input, &unmangled_name) {
        Ok(calling_convention) => Some(calling_convention),
        Err(e) => return failure(quote! { #input }, &e.to_string()),
    };

    let arg_types = input.sig.inputs.iter().map(|arg| match arg {
        FnArg::Receiver(_) => {
            quote! {compile_error!("Summoned functions cannot use the self parameter.")}
        }
        FnArg::Typed(pat) => {
            let ty = &pat.ty;
            quote! { #ty }
        }
    });

    let return_type = &input.sig.output;
    let vis = &input.vis;
    let attr = &input.attrs;
    let attr = quote! { #(#attr)* };

    let name = &input.sig.ident;
    let function_type = quote! { unsafe #calling_convention fn(#(#arg_types),*) #return_type };

    CPP_IMPORTS
        .lock()
        .expect("Failed to lock cpp imports")
        .push((mangled_name.clone(), name.to_string()));

    Diagnostic::spanned(Span::call_site(), Level::Note, unmangled_name.clone()).emit();

    quote! {
        #[allow(non_upper_case_globals)]
        #[allow(missing_docs)]
        #attr #vis static mut #name: rivets::UnsafeSummonedFunction<#function_type> = rivets::UnsafeSummonedFunction::Uninitialized;
    }.into()
}

fn get_hooks() -> Vec<proc_macro2::TokenStream> {
    MANGLED_NAMES
        .lock()
        .expect("Failed to lock mangled names")
        .iter()
        .map(|(mangled_name, module_name)| {
            let module_name = Ident::new(module_name, proc_macro2::Span::call_site());
            quote! {
                hooks.push(
                    rivets::RivetsHook {
                        mangled_name: #mangled_name.into(),
                        hook: #module_name::hook
                    }
                );
            }
        })
        .collect()
}

fn get_imports() -> Vec<proc_macro2::TokenStream> {
    CPP_IMPORTS.lock().expect("Failed to lock cpp imports")
        .iter()
        .map(|(mangled_name, rust_name)| {
            let rust_name = Ident::new(rust_name, proc_macro2::Span::call_site());
            quote! {
                let Some(address) = symbol_cache.get_function_address(base_address, #mangled_name)
                else {
                    panic!(
                        "Failed to find address for the following mangled function inside the PDB: {}", #mangled_name
                    );
                };
                let function = unsafe {
                    std::mem::transmute(address) // todo: rust documentation recommends casting this to a raw function pointer. address as *const _
                };
                unsafe { #rust_name = rivets::UnsafeSummonedFunction::Function(function); }
            }
        })
        .collect()
}

/// A procedural macro for finalizing the rivets library.
/// This macro should be called once at the end of the `main.rs` file.
/// It will finalize the rivets library and inject all of the detours.
#[proc_macro]
pub fn finalize(_: TokenStream) -> TokenStream {
    check_finalized!();
    IS_FINALIZED.store(true, std::sync::atomic::Ordering::Relaxed);

    let hooks = get_hooks();
    let imports = get_imports();

    let finalize = quote! {
        fn rivets_finalize(symbol_cache: rivets::SymbolCache) -> Option<String> {
            let base_address = match symbol_cache.get_module_base_address() {
                Ok(base_address) => base_address,
                Err(e) => return Some(format!("{e}")),
            };

            #(#imports)*

            let mut hooks: Vec<rivets::RivetsHook> = Vec::new();
            #(#hooks)*
            for hook in &hooks {
                let inject_result = unsafe { symbol_cache.inject(base_address, hook) };
                if inject_result.is_err() {
                    return Some(format!("{inject_result:?}"));
                }
            }
            None
        }
    };

    quote! { rivets::dll_syringe::payload_procedure! { #finalize } }.into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(factorio_define))]
struct DefineOpts {
    kind: Ident,
}

/// A procedural macro for constructing the factorio defines table.
#[proc_macro_derive(FactorioDefine, attributes(factorio_define, value))]
pub fn define_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let Ok(DefineOpts { kind }) = DefineOpts::from_derive_input(&input) else {
        return derive_error!("Missing #[kind(?)] attribute!");
    };
    let DeriveInput { ident, data, .. } = input;

    let syn::Data::Enum(data) = data else {
        return derive_error!("FactorioDefine can only be used on enums!");
    };

    let count = data.variants.len();
    let mut deref_matches = TokenStream2::new();
    let mut from_matches = TokenStream2::new();
    let mut variants = TokenStream2::new();

    for variant in data.variants {
        let mut value = None;
        for attr in &variant.attrs {
            let syn::Meta::NameValue(nv) = &attr.meta else {
                continue;
            };

            if !nv.path.is_ident::<str>("value") {
                continue;
            }

            let Expr::Lit(syn::PatLit { lit, .. }) = &nv.value else {
                return derive_error!("All variants must have a #[value(?)] attribute!");
            };

            value = Some(lit.clone());

            break;
        }

        let Some(value) = value else {
            return derive_error!("All variants must have a #[value = ?] attribute!");
        };

        let Variant { ident, .. } = variant;

        deref_matches.extend(std::iter::once(quote! {
            Self::#ident => &#value,
        }));

        from_matches.extend(std::iter::once(quote! {
            #value => Ok(Self::#ident),
        }));

        variants.extend(std::iter::once(quote! {
            Self::#ident,
        }));
    }

    let str_kind = format!("{kind}");
    let mut impl_from = {
        let kind = if str_kind == "str" {
            quote! {
                &#kind
            }
        } else {
            quote! {
                #kind
            }
        };

        quote! {
            impl std::convert::TryFrom<#kind> for #ident {
                type Error = &'static str;

                fn try_from(value: #kind) -> Result<Self, Self::Error> {
                    match value {
                        #from_matches
                        _ => Err("Invalid value"),
                    }
                }
            }
        }
    };

    if str_kind == "i8"
        || str_kind == "i16"
        || str_kind == "i32"
        || str_kind == "i64"
        || str_kind == "i128"
    {
        impl_from = quote! {
            #impl_from

            impl std::convert::TryFrom<&isize> for #ident {
                type Error = &'static str;

                fn try_from(value: &isize) -> Result<Self, Self::Error> {
                    match value {
                        #from_matches
                        _ => Err("Invalid value"),
                    }
                }
            }
        };
    } else if str_kind == "u8"
        || str_kind == "u16"
        || str_kind == "u32"
        || str_kind == "u64"
        || str_kind == "u128"
    {
        impl_from = quote! {
            #impl_from

            impl std::convert::TryFrom<&usize> for #ident {
                type Error = &'static str;

                fn try_from(value: &usize) -> Result<Self, Self::Error> {
                    match value {
                        #from_matches
                        _ => Err("Invalid value"),
                    }
                }
            }
        };
    }

    let output = quote! {
        impl std::ops::Deref for #ident {
            type Target = #kind;

            fn deref(&self) -> &'static #kind {
                match self {
                    #deref_matches
                }
            }
        }

        #impl_from

        impl Define<#count> for #ident {
            fn variants() -> &'static [Self; #count] {
                &[
                    #variants
                ]
            }
        }
    };

    output.into()
}
