#![feature(proc_macro_diagnostic)]

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

#[proc_macro_attribute]
pub fn detour(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mangled_name = attr.to_string();
    let unmangled_name =
        rivets_shared::demangle(&mangled_name).unwrap_or_else(|| mangled_name.clone());

    let mut input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let return_type = &input.sig.output;

    let inputs = &input.sig.inputs;
    let mut arg_names : Vec<proc_macro2::TokenStream> = vec![];
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

    let source = quote! {
        unsafe fn source(#inputs) #return_type {
            Detour.call(#arg_names)
        }
    };

    let result = quote! {
        retour::static_detour! {
            static Detour : #cpp_function_header;
        }

        #source

        #[doc = #unmangled_name]
        #callback

        unsafe fn hook(address: u64) -> anyhow::Result<()> {
            let compiled_function: #cpp_function_header = std::mem::transmute(address);
            Detour.initialize(compiled_function, #name)?.enable()?;
            Ok(())
        }

        #[ctor::ctor]
        fn ctor() {
            rivets::start_stream();
            if let Err(e) = rivets::inject(#mangled_name, hook) {
                tracing::error!("{e}");
            }
        }
    };

    Diagnostic::spanned(Span::call_site(), Level::Note, unmangled_name.clone()).emit();

    result.into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(factorio_define))]
struct DefineOpts {
    kind: Ident,
}

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

        variants.extend(std::iter::once(quote! {
            Self::#ident,
        }));
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
