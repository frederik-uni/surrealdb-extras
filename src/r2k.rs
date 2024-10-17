use crate::RecordIdType;
use std::any::type_name;
use std::collections::{HashMap, HashSet};
use surrealdb::sql::Kind;

//todo: make static
fn register() -> HashMap<&'static str, Kind> {
    let mut hm = HashMap::new();
    hm.insert(type_name::<bool>(), Kind::Bool);
    hm.insert(type_name::<i8>(), Kind::Int);
    hm.insert(type_name::<i16>(), Kind::Int);
    hm.insert(type_name::<i32>(), Kind::Int);
    hm.insert(type_name::<i64>(), Kind::Int);
    hm.insert(type_name::<i128>(), Kind::Int);
    hm.insert(type_name::<isize>(), Kind::Int);
    hm.insert(type_name::<u8>(), Kind::Int);
    hm.insert(type_name::<u16>(), Kind::Int);
    hm.insert(type_name::<u32>(), Kind::Int);
    hm.insert(type_name::<u64>(), Kind::Int);
    hm.insert(type_name::<u128>(), Kind::Int);
    hm.insert(type_name::<usize>(), Kind::Int);
    hm.insert(type_name::<f32>(), Kind::Float);
    hm.insert(type_name::<f64>(), Kind::Float);
    hm.insert(type_name::<String>(), Kind::String);
    hm.insert(type_name::<&str>(), Kind::String);
    hm.insert(type_name::<surrealdb::sql::Strand>(), Kind::String);
    hm.insert(type_name::<surrealdb::sql::Uuid>(), Kind::Uuid);
    hm.insert(type_name::<surrealdb::sql::Thing>(), Kind::Record(vec![]));
    hm.insert(type_name::<crate::RecordIdFunc>(), Kind::Record(vec![]));
    hm.insert(type_name::<surrealdb::sql::Bytes>(), Kind::Bytes);
    hm.insert(type_name::<surrealdb::sql::Number>(), Kind::Number);
    hm.insert(type_name::<surrealdb::sql::Object>(), Kind::Object);
    hm.insert(
        type_name::<surrealdb::sql::Array>(),
        Kind::Array(Box::new(Kind::Any), None),
    );
    hm.insert(
        type_name::<surrealdb::sql::Geometry>(),
        Kind::Geometry(vec![]),
    );
    hm.insert(type_name::<surrealdb::sql::Datetime>(), Kind::Datetime);
    hm.insert(type_name::<surrealdb::sql::Duration>(), Kind::Duration);
    hm.insert(type_name::<surrealdb::sql::Operation>(), Kind::Object);
    #[cfg(feature = "rust_decimal")]
    hm.insert(type_name::<rust_decimal::Decimal>(), Kind::Decimal);
    #[cfg(feature = "geo")]
    hm.insert(
        type_name::<geo::Point<f64>>(),
        Kind::Geometry(vec!["point".to_string()]),
    );
    #[cfg(feature = "chrono")]
    hm.insert(type_name::<chrono::DateTime<chrono::Utc>>(), Kind::Datetime);
    #[cfg(feature = "uuid")]
    hm.insert(type_name::<uuid::Uuid>(), Kind::Uuid);
    hm.insert(
        type_name::<(f64, f64)>(),
        Kind::Geometry(vec!["line".to_string()]),
    );
    hm.insert(
        type_name::<[f64; 2]>(),
        Kind::Geometry(vec!["line".to_string()]),
    );
    hm
}

pub fn to_kind(s: &str, names: &HashMap<&'static str, &'static str>) -> Kind {
    let items = register();
    if let Some(v) = items.get(s) {
        return v.clone();
    }
    if let Some(v) = parse_inner::<Vec<bool>>(s) {
        Kind::Array(Box::new(to_kind(v.as_str(), names)), None)
    } else if let Some(v) = parse_inner::<RecordIdType<bool>>(s) {
        let db = names
            .get(v.as_str())
            .unwrap_or_else(|| panic!("{} is not a table", v));
        Kind::Record(vec![surrealdb::sql::Table::from(db.to_string())])
    } else if let Some(v) = parse_inner::<Option<bool>>(s) {
        let ki = to_kind(v.as_str(), names);
        if ki == Kind::Any {
            Kind::Any
        } else {
            Kind::Option(Box::new(ki))
        }
    } else if parse_inner::<HashMap<bool, bool>>(s).is_some() {
        Kind::Object
    } else if let Some(v) = parse_inner::<HashSet<bool>>(s) {
        Kind::Set(Box::new(to_kind(v.as_str(), names)), None)
    } else {
        println!("{}", s);
        Kind::Any
    }
}

fn parse_inner<T>(value: &str) -> Option<String> {
    let start = format!(
        "{}<",
        type_name::<T>()
            .split_once('<')
            .expect("T should have a generic")
            .0
    );
    if value.starts_with(&start) && value.ends_with('>') {
        Some(
            value[..value.len() - 1]
                .strip_prefix(&start)
                .unwrap()
                .to_string(),
        )
    } else {
        None
    }
}
