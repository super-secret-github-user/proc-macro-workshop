use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, parse_macro_input, DeriveInput};

//     pub struct CommandBuilder {
//         executable: Option<String>,
//         args: Option<Vec<String>>,
//         env: Option<Vec<String>>,
//         current_dir: Option<String>,
//     }
//
// and in the `builder` function:
//
//     impl Command {
//         pub fn builder() -> CommandBuilder {
//             CommandBuilder {
//                 executable: None,
//                 args: None,
//                 env: None,
//                 current_dir: None,
//             }
//         }
//     }

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let builder_name = format_ident!("{}Builder", ident);

    let strct = match data {
      Data::Struct(s) => s,
      _ => unimplemented!()
    };

    let wrapped_types = strct.fields.iter().map(|it| {
      let field_ident = it.ident.as_ref().expect("only supports named fields");
      let field_type = &it.ty;

      quote! {
        #field_ident: Option<#field_type>
      }
    });

    (quote! {
      #[derive(Default)]
      pub struct #builder_name {
				#(#wrapped_types,)*
      }

      impl #ident {
        pub fn builder() -> #builder_name {
          #builder_name::default()
        }
      }
    })
    .into()
}
