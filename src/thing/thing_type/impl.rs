use crate::{ThingFunc, ThingType};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use surrealdb::sql::Thing;

impl<T> Debug for ThingType<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", type_name::<T>())?;
        self.thing.fmt(f)
    }
}

impl<T> Serialize for ThingType<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.thing.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for ThingType<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            thing: ThingFunc::new(Thing::deserialize(deserializer)?),
            parse_to: Default::default(),
        })
    }
}
