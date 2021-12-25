use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

#[macro_use]
extern crate quote;
extern crate proc_macro;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(relay))]
struct RelayNodeObjectAttributes {
    node_suffix: Option<String>,
}

/// The RelayNodeObject macro is applied to a type to automatically implement the RelayNodeStruct trait.
/// ```
/// #[derive(SimpleObject, RelayNodeObject)] // See the 'RelayNodeObject' derive macro
/// #[graphql(complex)]
/// #[relay(node_suffix = "u")] // This controls the 'RelayNodeObject' macro. In this case the prefix is shortened to 'u', the default is in the name of the struct.
/// pub struct User {
///     pub id: RelayNodeID<User>,
///     pub name: String,
///     pub role: String,
/// }
/// ```
#[proc_macro_derive(RelayNodeObject, attributes(relay))]
pub fn derive_relay_node_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let attrs = RelayNodeObjectAttributes::from_derive_input(&input)
        .expect("Error parsing 'RelayNodeObject' macro options!");
    let DeriveInput { ident, data, .. } = input;

    if !matches!(data, Data::Struct(_)) {
        panic!("The 'RelayNodeObject' macro can only be used on structs!");
    }

    let value = if let Some(node_suffix) = attrs.node_suffix {
        node_suffix
    } else {
        ident.to_string()
    };

    quote! {
        impl async_graphql_relay::RelayNodeStruct for #ident {
            const ID_SUFFIX: &'static str = #value;
        }
    }
    .into()
}

/// The RelayInterface macro is applied to a GraphQL Interface enum to allow it to be used for Relay's node query.
/// This enum should contain all types that that exist in your GraphQL schema to work as designed in the Relay server specification.
/// ```
/// #[derive(Interface, RelayInterface)] // See the 'RelayInterface' derive macro
/// #[graphql(field(name = "id", type = "NodeGlobalID"))] // The 'RelayInterface' macro generates a type called '{enum_name}GlobalID' which should be used like this to facilitate using the async_graphql_relay::RelayNodeID for globally unique ID's
/// pub enum Node {
///     User(User),
///     Tenant(Tenant),
///    // Put all of your Object's in this enum
/// }
/// ```
#[proc_macro_derive(RelayInterface)]
pub fn derive_relay_interface(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let ident = format_ident!("{}GlobalID", ident);
    let impls;
    let node_matchers;
    if let Data::Enum(data) = &data {
        impls = data.variants.iter().map(|variant| {
            let variant_ident = &variant.ident;
            quote! {
                impl std::convert::From<&RelayNodeID<#variant_ident>> for #ident {
                    fn from(t: &RelayNodeID<#variant_ident>) -> Self {
                        #ident(String::from(t))
                    }
                }
            }
        });

        node_matchers = data.variants.iter().map(|variant| {
            let variant_ident = &variant.ident;
            quote! {
                <#variant_ident as async_graphql_relay::RelayNodeStruct>::ID_SUFFIX => {
                    #variant_ident::get(
                        ctx,
                        async_graphql_relay::RelayNodeID::<#variant_ident>::new_from_relay_id(
                            relay_id.to_string(),
                        )?,
                    )
                    .await?
                    .ok_or_else(|| Error::new("A node with the specified id could not be found!"))
                }
            }
        });
    } else {
        panic!("The 'RelayNodeObject' macro can only be used on enums!");
    }

    quote! {
                #[derive(Clone, Debug)]
        pub struct #ident(String);

        #(#impls)*

        #[async_graphql::Scalar(name = "RelayNodeID")]
        impl async_graphql::ScalarType for #ident {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                unimplemented!();
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(self.0.clone())
            }
        }

        #[async_graphql_relay::_async_trait]
        impl async_graphql_relay::RelayNodeInterface for Node {
            async fn fetch_node(ctx: async_graphql_relay::RelayContext, relay_id: String) -> Result<Self, async_graphql::Error> {
                if relay_id.len() < 32 {
                    return Err(Error::new("Invalid id provided to node query!"));
                }
                let (_, suffix) = relay_id.split_at(32);
                match suffix {
                    #(#node_matchers)*
                    _ => Err(Error::new("A node with the specified id could not be found!")),
                }
            }
        }
            }
    .into()
}

// TODO: Unit tests
