use crate::RecordIdFunc;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

impl Serialize for RecordIdFunc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RecordIdFunc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::new(RecordId::deserialize(deserializer)?))
    }
}
