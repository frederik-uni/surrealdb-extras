use crate::{Record, RecordData, SurrealTableInfo, ThingFunc, ThingType};
use std::str::FromStr;
use surrealdb::sql::{Id, Strand, Thing};
use surrealdb::syn;

impl<T: SurrealTableInfo> From<ThingFunc> for ThingType<T> {
    fn from(value: ThingFunc) -> Self {
        Self::new(value)
    }
}

impl<T: SurrealTableInfo> From<Thing> for ThingType<T> {
    fn from(value: Thing) -> Self {
        Self::new_thing(value)
    }
}

impl<T: SurrealTableInfo> From<Record> for ThingType<T> {
    fn from(value: Record) -> Self {
        Self::from(value.id)
    }
}

impl<T: SurrealTableInfo> From<RecordData<T>> for ThingType<T> {
    fn from(value: RecordData<T>) -> Self {
        Self::from(value.id)
    }
}

impl<T: SurrealTableInfo> From<(&str, Id)> for ThingType<T> {
    fn from(value: (&str, Id)) -> Self {
        Self::from(Thing::from(value))
    }
}

impl<T: SurrealTableInfo> From<(String, Id)> for ThingType<T> {
    fn from(value: (String, Id)) -> Self {
        Self::from(Thing::from(value))
    }
}

impl<T: SurrealTableInfo> From<(String, String)> for ThingType<T> {
    fn from(value: (String, String)) -> Self {
        Self::from(Thing::from(value))
    }
}

impl<T: SurrealTableInfo> From<(&str, &str)> for ThingType<T> {
    fn from(value: (&str, &str)) -> Self {
        Self::from(Thing::from(value))
    }
}

impl<T: SurrealTableInfo> FromStr for ThingType<T> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(Thing::try_from(s)?))
    }
}

impl<T: SurrealTableInfo> TryFrom<String> for ThingType<T> {
    type Error = ();
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Ok(Self::from(Thing::try_from(v.as_str())?))
    }
}

impl<T: SurrealTableInfo> TryFrom<Strand> for ThingType<T> {
    type Error = ();
    fn try_from(v: Strand) -> Result<Self, Self::Error> {
        Ok(Self::from(Thing::try_from(v.as_str())?))
    }
}

impl<T: SurrealTableInfo> TryFrom<&str> for ThingType<T> {
    type Error = ();
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match syn::thing(v) {
            Ok(v) => Ok(Self::from(v)),
            _ => Err(()),
        }
    }
}
