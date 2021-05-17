extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[macro_use]
extern crate quote;

#[proc_macro_derive(RelayGlobalID)]
pub fn derive_relay_global_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let m = quote! {
        impl #name {
            pub fn relay_id(&self) -> String {
                format!("{}{}", self.id, SchemaNodeTypes::#name as u32)
            }
        }
    };

    TokenStream::from(m)
}

#[proc_macro_derive(RelayNodeEnum)]
pub fn derive_relay_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let variants = match input.data {
        Data::Enum(e) => e
            .variants
            .into_iter()
            .map(|v| v.ident)
            .collect::<Vec<Ident>>(),
        _ => {
            panic!("The RelayNode macro must be used on an enum type");
        }
    };
    let variant_node_type = (0..variants.len()).map(|v| (v + 1).to_string());

    let m = quote! {
        pub enum SchemaNodeTypes {
            Unknown = 0,
            #(
                #variants,
            )*
        }

        impl #name {
            pub async fn get(relay_id: String) -> Option<Node> {
                if relay_id.len() < 36 {
                    None?
                }
                let (id, node_type) = relay_id.split_at(36);
                match node_type {
                    #(
                        #variant_node_type => Some(<#variants>::get(id.to_string()).await),
                    )*
                    _ => None
                }
            }
        }
    };

    TokenStream::from(m)
}
