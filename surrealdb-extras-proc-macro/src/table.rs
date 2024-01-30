use crate::{SurrealDatabaseName, SurrealTableOverwrite, SurrealDatabaseExtraCommands};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive_attribute_collector(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let SurrealDatabaseExtraCommands(temp) = deluxe::extract_attributes(&mut input).unwrap_or_default();
    let SurrealDatabaseName(struct_name) = match deluxe::extract_attributes(&mut input) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };


    let struct_impl = &input.ident;
    let attributes: Vec<(TokenStream2, Option<String>, Option<String>)> = {
        match &mut input.data {
            Data::Struct(data_struct) => match &mut data_struct.fields {
                Fields::Named(fields_named) => {
                    let mut items = vec![(
                        quote! {
                                format!("DEFINE TABLE {};", #struct_name)
                        },
                        None,
                        None,
                    )];
                    items.append(
                        &mut fields_named
                            .named
                            .iter_mut()
                            .map(|field| {
                                let renamed: Option<SurrealTableOverwrite> =
                                    match deluxe::parse_attributes(field) {
                                        Ok(obj) => Some(obj),
                                        Err(_) => None,
                                    };

                                let (field_name, field_type, exclude) = renamed.map(|v| (v.rename.clone(), v.db_type.clone(), v.exclude)).unwrap_or((None, None, None));
                                let field_name = field_name.unwrap_or(field.ident.as_ref().unwrap().to_string());
                                (match field_type {
                                    Some(field_type) => {
                                        quote! {
                                           format!("DEFINE FIELD {} ON TABLE {} TYPE {};",#field_name, #struct_name, #field_type)
                                        }
                                    }
                                    None => {
                                        let ty = &field.ty;
                                        quote! {
                                           format!("DEFINE FIELD {} ON TABLE {} TYPE {};",#field_name, #struct_name, surrealdb_extras::rust_to_surreal(std::any::type_name::<#ty>(), names))
                                        }
                                    }
                                }, exclude.and_then(|v|match v {
                                    true => Some(field_name.clone()),
                                    false => None
                                }), Some(field_name))
                            })
                            .collect::<Vec<_>>()
                    );
                    items
                }
                _ => unimplemented!("AttributeCollector only supports named fields."),
            },
            _ => unimplemented!("AttributeCollector only supports structs."),
        }
    };

    let mut attr = vec![];
    let mut exc = vec![];
    let mut keys = vec![];
    for (tk, opt, key) in attributes {
        attr.push(tk);
        if let Some(v) = opt {
            exc.push(v);
        }
        if let Some(v) = key {
            keys.push(v);
        }
    }
    for temp in temp {
        attr.push(quote!(#temp.to_string()));
    }

    let gen = quote! {
        impl surrealdb_extras::SurrealSelectInfo for #struct_impl {
            fn keys()-> &'static [&'static str] {
                &[#( #keys ),*]
            }
        }

        impl surrealdb_extras::SurrealTableInfo for #struct_impl {
            fn name() -> &'static str {
                #struct_name
            }

            fn path() -> &'static str {
                std::any::type_name::<#struct_impl>()
            }

            fn exclude() -> &'static [&'static str] {
                &[#( #exc ),*]
            }

            fn funcs(names: &std::collections::HashMap<&'static str, &'static str>) ->  Vec<String>{
                vec![#( #attr ),*]
            }
        }

        impl #struct_impl {
            pub fn add<'a: 'b, 'b, D: surrealdb::Connection>(self, conn: &'a surrealdb::Surreal<D>)-> surrealdb::method::Content<'b, D, #struct_impl, Vec<surrealdb_extras::RecordData<#struct_impl>>> {
                conn.create(#struct_name).content(self)
            }

            pub fn insert<'a: 'b, 'b, D: surrealdb::Connection>(self, conn: &'a surrealdb::Surreal<D>, id: surrealdb::sql::Id)-> surrealdb::method::Content<'b, D, #struct_impl, Option<surrealdb_extras::RecordData<#struct_impl>>> {
                conn.create((#struct_name, id)).content(self)
            }
        }
    };
    gen.into()
}
