use proc_macro::{TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

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
//     impl CommandBuilder {
//         fn executable(&mut self, executable: String) -> &mut Self {
//             self.executable = Some(executable);
//             self
//         }

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let builder_name = format_ident!("{}Builder", ident);

    let strct = match data {
        Data::Struct(s) => s,
        _ => unimplemented!(),
    };


    let wrapped_fields = strct.fields.iter().map(| it | {
      let ident = it.ident.as_ref().expect("named fields only");
      let ty = &it.ty;

      quote! {
        #ident: Option<#ty>
      }
    });

    let setter_methods = strct.fields.iter().map(| it | {
      let ident = it.ident.as_ref().expect("named fields only");
      let ty = &it.ty;

      quote! {
  			pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
    			self.#ident = Some(#ident);
    			self
  			}
      }
    });

    let expanded = quote! {
      #[derive(Default)]
      pub struct #builder_name {
        #(#wrapped_fields),*
      }

      impl #builder_name {
        #(#setter_methods)*
      }

      impl #ident {
        pub fn builder() -> #builder_name {
          #builder_name::default()
        }
      }
    };

    eprintln!("expanded: {}", expanded);

    expanded.into()
}
