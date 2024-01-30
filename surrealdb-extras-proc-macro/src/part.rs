use crate::SurrealTableOverwrite;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive_attribute_collector(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let mut attributes = vec!["id".to_string()];
    attributes.append(
        &mut {
            match &mut input.data {
                Data::Struct(data_struct) => match &mut data_struct.fields {
                    Fields::Named(fields_named) => fields_named.named.iter_mut().map(|field| {
                        let renamed: Option<SurrealTableOverwrite> =
                            match deluxe::parse_attributes(field) {
                                Ok(obj) => Some(obj),
                                Err(_) => None,
                            };
                        let field_name = renamed
                            .and_then(|v| v.rename.clone())
                            .unwrap_or(field.ident.as_ref().unwrap().to_string());
                        field_name
                    }),
                    _ => unimplemented!("AttributeCollector only supports structs."),
                },
                _ => unimplemented!("AttributeCollector only supports structs."),
            }
        }
        .collect::<Vec<_>>(),
    );
    let struct_impl = &input.ident;
    let gen = quote! {

        impl surrealdb_extras::SurrealSelectInfo for #struct_impl {
            fn keys() -> &'static [&'static str]{
                &[#( #attributes ),*]
            }
        }
    };

    gen.into()
}
