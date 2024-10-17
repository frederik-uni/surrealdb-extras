use crate::{RecordIdFunc, RecordIdType};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use surrealdb::RecordId;

impl<T> Debug for RecordIdType<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", type_name::<T>())?;
        self.thing.fmt(f)
    }
}

impl<T> Serialize for RecordIdType<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.thing.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for RecordIdType<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            thing: RecordIdFunc::new(RecordId::deserialize(deserializer)?),
            parse_to: Default::default(),
        })
    }
}
