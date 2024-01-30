mod part;
mod table;

use proc_macro::TokenStream;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

use quote::quote;
use std::fs::File;
use std::io::Write;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[derive(deluxe::ExtractAttributes, deluxe::ParseAttributes)]
#[deluxe(attributes(serde))]
struct SerdeRename {
    rename: String,
}

//TODO: apply rename to serde
#[derive(deluxe::ExtractAttributes, deluxe::ParseAttributes)]
#[deluxe(attributes(opt))]
struct SurrealTableOverwrite {
    rename: Option<String>,
    db_type: Option<String>,
    exclude: Option<bool>
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(db))]
struct SurrealDatabaseName(String);

#[derive(deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(sql))]
struct SurrealDatabaseExtraCommands(Vec<String>);

#[proc_macro_derive(SurrealSelect, attributes(opt))]
/// implements SurrealSelectInfo
pub fn select(input: TokenStream) -> TokenStream {
    part::derive_attribute_collector(input)
}

#[proc_macro_derive(SurrealTable, attributes(db, opt, sql))]
/// implements SurrealSelectInfo, SurrealTableInfo, add and insert
pub fn table(input: TokenStream) -> TokenStream {
    table::derive_attribute_collector(input)
}
