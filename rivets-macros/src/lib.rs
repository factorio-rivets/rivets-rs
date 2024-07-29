use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn};

#[proc_macro_attribute]
pub fn detour(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mangled_name = attr.to_string();

    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let return_type = &input.sig.output;

    let arguments: Vec<proc_macro2::TokenStream> = input
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => {
                quote! {compile_error!("Detour functions cannot use the self parameter.")}
            }
            FnArg::Typed(pat) => {
                let attrs = &pat.attrs;
                let ty = &pat.ty;
                quote! { #(#attrs)* #ty }
            }
        })
        .collect();
    let arguments = quote! { #( #arguments ),* };

    let cpp_function_header = quote! {
        unsafe extern "C" fn(#arguments) #return_type
    };

    let callback = quote! { #input };

    quote! {
        #callback

        retour::static_detour! {
            static MainHook: #cpp_function_header;
        }

        unsafe fn hook(address: u64) -> anyhow::Result<()> {
            let compiled_function: #cpp_function_header = std::mem::transmute(address);
            MainHook.initialize(compiled_function, #name)?.enable()?;
            Ok(())
        }

        #[ctor::ctor]
        fn ctor() {
            rivets::start_stream();
            if let Err(e) = rivets::inject(#mangled_name, hook) {
                tracing::error!("{e}");
            }
        }
    }
    .into()
}
