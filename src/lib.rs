use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[macro_use]
extern crate quote;
extern crate proc_macro;

#[proc_macro_derive(RelayGlobalID, attributes(relay_global_id))]
pub fn derive_relay_global_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let m = quote! {
        impl From<&#name> for String {
            fn from(id: &#name) -> Self {
                let node_type = id.1.clone() as u32;
                let mut uuid = id.0.clone();
                if uuid.len() < 36 {
                    panic!("ID type must only contain a UUIDv4");
                }
                uuid.remove(8);
                uuid.remove(12);
                uuid.remove(16);
                uuid.remove(20);
                format!("{}{}", uuid, node_type)
            }
        }
        #[Scalar]
        impl ScalarType for #name {
            fn parse(_value: Value) -> InputValueResult<Self> {
                unimplemented!();
            }
            fn to_value(&self) -> Value {
                Value::String(String::from(self))
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
        #[derive(Clone)]
        pub enum SchemaNodeTypes {
            Unknown = 0,
            #(
                #variants,
            )*
        }

        impl #name {
            pub async fn get(relay_id: String) -> Option<Node> {
                if relay_id.len() < 32 {
                    None?
                }
                let (id, node_type) = relay_id.split_at(32);
                let mut id = id.to_string();
                id.insert(8, '-');
                id.insert(13, '-');
                id.insert(18, '-');
                id.insert(23, '-');

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
