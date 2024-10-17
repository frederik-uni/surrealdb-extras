#![doc=include_str!( "../readme.md")]
mod define;
mod does_imp;
mod r2k;
mod records;
mod surreal_table;
pub mod thing;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub use define::use_ns_db;
pub use records::Record;
pub use records::RecordData;
pub use surreal_table::SurrealTableInfo;
pub use surrealdb_extras_proc_macro::SurrealSelect;
pub use surrealdb_extras_proc_macro::SurrealTable;
pub use thing::RecordIdFunc;
pub use thing::RecordIdType;
pub use thing::ThingArray;

#[doc(hidden)]
/// converts struct structure to the db type
/// is used by SurrealTableInfo
pub fn rust_to_surreal(s: &str, names: &HashMap<&'static str, &'static str>) -> String {
    r2k::to_kind(s, names).to_string()
}

/// SELECT {keys} IN db
pub trait SurrealSelectInfo: DeserializeOwned {
    /// all attributes
    fn keys() -> &'static [&'static str];
}
