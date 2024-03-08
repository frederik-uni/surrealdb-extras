use crate::{Record, RecordData, ThingFunc, ThingType};
use std::str::FromStr;
use surrealdb::sql::{Id, Strand, Thing};
use surrealdb::syn;

impl From<Thing> for ThingFunc {
    fn from(value: Thing) -> Self {
        ThingFunc::new(value)
    }
}

impl<T> From<ThingType<T>> for ThingFunc {
    fn from(value: ThingType<T>) -> Self {
        Self(value.thing.0)
    }
}

impl From<Record> for ThingFunc {
    fn from(value: Record) -> Self {
        value.id
    }
}

impl<T> From<RecordData<T>> for ThingFunc {
    fn from(value: RecordData<T>) -> Self {
        value.id
    }
}

impl From<(&str, Id)> for ThingFunc {
    fn from(value: (&str, Id)) -> Self {
        Self::from(Thing::from(value))
    }
}

impl From<(String, Id)> for ThingFunc {
    fn from(value: (String, Id)) -> Self {
        Self::from(Thing::from(value))
    }
}

impl From<(String, String)> for ThingFunc {
    fn from(value: (String, String)) -> Self {
        Self::from(Thing::from(value))
    }
}

impl From<(&str, &str)> for ThingFunc {
    fn from(value: (&str, &str)) -> Self {
        Self::from(Thing::from(value))
    }
}

impl FromStr for ThingFunc {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(Thing::try_from(s)?))
    }
}

impl TryFrom<String> for ThingFunc {
    type Error = ();
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Ok(Self::from(Thing::try_from(v.as_str())?))
    }
}

impl TryFrom<Strand> for ThingFunc {
    type Error = ();
    fn try_from(v: Strand) -> Result<Self, Self::Error> {
        Ok(Self::from(Thing::try_from(v.as_str())?))
    }
}

impl TryFrom<&str> for ThingFunc {
    type Error = ();
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match syn::thing(v) {
            Ok(v) => Ok(Self::from(v)),
            _ => Err(()),
        }
    }
}
