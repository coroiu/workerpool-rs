extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn global_routine(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    // let fn_name_str = fn_name.to_string();

    // Generate the routine registration
    let register_fn_name = format_ident!("register_routine_{}", fn_name);
    let expanded = quote! {
        #input

        #[ctor::ctor]
        fn #register_fn_name() {
            let routine = Routine::new(#fn_name);
            register_routine(routine);
        }
    };

    TokenStream::from(expanded)
}
