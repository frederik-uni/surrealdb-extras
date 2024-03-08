use crate::thing::thing_array::ThingArray;
use crate::{ThingFunc, ThingType};
use surrealdb::sql::Thing;

impl From<Vec<ThingFunc>> for ThingArray {
    fn from(value: Vec<ThingFunc>) -> Self {
        Self(value.into_iter().map(|v| v.0).collect())
    }
}

impl<T> From<Vec<ThingType<T>>> for ThingArray {
    fn from(value: Vec<ThingType<T>>) -> Self {
        Self(value.into_iter().map(|v| v.thing.0).collect())
    }
}

impl From<Vec<Thing>> for ThingArray {
    fn from(value: Vec<Thing>) -> Self {
        Self(value)
    }
}
