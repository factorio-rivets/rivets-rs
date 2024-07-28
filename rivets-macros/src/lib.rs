use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn detour(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let mangled_name = attr.to_string();

    let name = &input.sig.ident;
    let block = &input.block;

    let output = quote! {
        static_detour! {
            static MainHook: unsafe extern "C" fn() -> bool;
        }

        type FnMain = unsafe extern "C" fn() -> bool;

        fn main_detour() -> bool {
            info!("Detoured into main!");
            false
        }

        unsafe fn hook(address: u64) -> Result<()> {
            let fnmain: FnMain = std::mem::transmute(address);
            MainHook.initialize(fnmain, main_detour)?.enable()?;
            Ok(())
        }

        #[ctor::ctor]
        fn ctor() {
            start_stream();

            if let Err(e) = inject("?valid@LuaSurface@@UEBA_NXZ", hook) {
                error!("{e}");
            }
        }
    };

    TokenStream::from(output)
}
