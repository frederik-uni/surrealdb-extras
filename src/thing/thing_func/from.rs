use crate::{Record, RecordData, RecordIdFunc, RecordIdType};
use std::str::FromStr;
use surrealdb::sql::Strand;
use surrealdb::{RecordId, RecordIdKey};

impl From<RecordId> for RecordIdFunc {
    fn from(value: RecordId) -> Self {
        RecordIdFunc::new(value)
    }
}

impl<T> From<RecordIdType<T>> for RecordIdFunc {
    fn from(value: RecordIdType<T>) -> Self {
        Self(value.thing.0)
    }
}

impl From<Record> for RecordIdFunc {
    fn from(value: Record) -> Self {
        value.id
    }
}

impl<T> From<RecordData<T>> for RecordIdFunc {
    fn from(value: RecordData<T>) -> Self {
        value.id
    }
}

impl From<(&str, RecordIdKey)> for RecordIdFunc {
    fn from(value: (&str, RecordIdKey)) -> Self {
        Self::from(RecordId::from(value))
    }
}

impl From<(String, RecordIdKey)> for RecordIdFunc {
    fn from(value: (String, RecordIdKey)) -> Self {
        Self::from(RecordId::from(value))
    }
}

impl From<(String, String)> for RecordIdFunc {
    fn from(value: (String, String)) -> Self {
        Self::from(RecordId::from(value))
    }
}

impl From<(&str, &str)> for RecordIdFunc {
    fn from(value: (&str, &str)) -> Self {
        Self::from(RecordId::from(value))
    }
}

impl FromStr for RecordIdFunc {
    type Err = surrealdb::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(RecordId::from_str(s)?))
    }
}

impl TryFrom<String> for RecordIdFunc {
    type Error = surrealdb::Error;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Ok(Self::from(RecordId::from_str(v.as_str())?))
    }
}

impl TryFrom<Strand> for RecordIdFunc {
    type Error = surrealdb::Error;
    fn try_from(v: Strand) -> Result<Self, Self::Error> {
        Ok(Self::from(RecordId::from_str(v.as_str())?))
    }
}

impl TryFrom<&str> for RecordIdFunc {
    type Error = surrealdb::Error;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        Ok(Self::from(RecordId::from_str(v)?))
    }
}
