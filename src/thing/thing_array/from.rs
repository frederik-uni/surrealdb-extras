use crate::thing::thing_array::ThingArray;
use crate::{RecordIdFunc, RecordIdType};
use surrealdb::RecordId;

impl From<Vec<RecordIdFunc>> for ThingArray {
    fn from(value: Vec<RecordIdFunc>) -> Self {
        Self(value.into_iter().map(|v| v.0).collect())
    }
}

impl<T> From<Vec<RecordIdType<T>>> for ThingArray {
    fn from(value: Vec<RecordIdType<T>>) -> Self {
        Self(value.into_iter().map(|v| v.thing.0).collect())
    }
}

impl From<Vec<RecordId>> for ThingArray {
    fn from(value: Vec<RecordId>) -> Self {
        Self(value)
    }
}
