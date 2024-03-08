use crate::ThingFunc;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

impl Serialize for ThingFunc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ThingFunc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::new(Thing::deserialize(deserializer)?))
    }
}
