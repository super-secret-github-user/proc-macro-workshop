use proc_macro::{TokenStream};
use quote::{quote};
use syn::{
    parse_macro_input, DeriveInput
};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(input as DeriveInput);

    let struct_name = tokens.ident;

    (quote! {
       impl #struct_name {
            pub fn builder() {}
        }
    }).into()
}
