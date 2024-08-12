#![warn(missing_docs)]
#![feature(proc_macro_diagnostic)]

//! Contains the proc macros for rivets.

use anyhow::{bail, Result};
use darling::FromDeriveInput;
use lazy_regex::regex;
use proc_macro::{self, Diagnostic, Level, Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Abi, DeriveInput, Error, Expr, FnArg, Ident, ItemFn, Variant};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(proc_macro2::Span::call_site(), $string)
            .to_compile_error()
            .into()
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

static mut MANGLED_NAMES: Vec<(String, String)> = vec![];

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

            pub unsafe extern "C" fn hook(address: u64) -> rivets::abi_stable::std_types::RResult<(), rivets::abi_stable::std_types::RBoxError> {
                unsafe fn hook(address: u64) -> Result<(), rivets::retour::Error> {
                    let compiled_function: #cpp_function_header = std::mem::transmute(address);
                    Detour.initialize(compiled_function, #name)?.enable()?;
                    Ok(())
                }
                hook(address).map_err(|e| rivets::abi_stable::std_types::RBoxError::new(e)).into()
            }
        }
    };

    unsafe {
        MANGLED_NAMES.push((mangled_name.clone(), format!("{name}")));
    }

    Diagnostic::spanned(Span::call_site(), Level::Note, unmangled_name.clone()).emit();

    result.into()
}

/// A procedural macro for initializing the rivets library.
/// This macro should be called once at the end of the `main.rs` file.
/// It will initialize the rivets library and inject all of the detours.
#[proc_macro]
pub fn initialize(_: TokenStream) -> TokenStream {
    let injects = unsafe { MANGLED_NAMES.clone() };
    let injects = injects.iter().map(|(mangled_name, name)| {
        let name = Ident::new(name, proc_macro2::Span::call_site());
        quote! {
            hooks.push(
                rivets::RivetsHook {
                    mangled_name: #mangled_name.into(),
                    hook: #name::hook
                }
            );
        }
    });

    let vec = quote! { abi_stable::std_types::RVec };

    quote! {
        #[rivets::abi_stable::sabi_extern_fn]
        pub extern "C" fn rivets_initialize() -> #vec<rivets::RivetsHook> {
            let mut hooks: #vec<rivets::RivetsHook> = #vec::new();
            #(#injects)*
            hooks
        }
    }
    .into()
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
