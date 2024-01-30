use crate::{Record, RecordData, SurrealSelectInfo};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use surrealdb::method::{Content, Delete, Merge, Patch, Select};
use surrealdb::opt::PatchOp;
use surrealdb::sql::{Id, Thing};
use surrealdb::{Connection, Error, Surreal};

/// ThingFunc + defining the table for SurrealTableInfo
/// ```
/// #[derive(surrealdb_extras::SurrealTable, serde::Serialize, serde::Deserialize)]
/// #[db("test_table")]
/// struct Test {
///     name: String,
///     /// a refrence to another entry in the table `test_table`
///     refr: surrealdb_extras::ThingType<Test>
/// }
/// ```
pub struct ThingType<T> {
    /// thing func
    pub thing: ThingFunc,
    /// should never be initialized
    ________optional_data_______: Option<Box<T>>,
}

impl<T> From<ThingFunc> for ThingType<T> {
    fn from(value: ThingFunc) -> Self {
        Self::new(value)
    }
}

impl<T> From<Thing> for ThingType<T> {
    fn from(value: Thing) -> Self {
        Self::new_thing(value)
    }
}

impl<T> ThingType<T> {
    pub fn new(thing_func: ThingFunc) -> Self {
        Self {
            thing: thing_func,
            ________optional_data_______: None,
        }
    }
    pub fn new_thing(thing: Thing) -> Self {
        Self {
            thing: ThingFunc::new(thing),
            ________optional_data_______: None,
        }
    }
    pub async fn get_part<C: Connection, TT: SurrealSelectInfo>(
        self,
        conn: &Surreal<C>,
    ) -> Result<Option<RecordData<TT>>, Error> {
        self.thing.get_part(conn).await
    }
}

impl<T> Debug for ThingType<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", type_name::<T>())?;
        self.thing.fmt(f)
    }
}

impl<T> Clone for ThingType<T> {
    fn clone(&self) -> Self {
        Self {
            thing: self.thing.clone(),
            ________optional_data_______: None,
        }
    }
}

impl<T> PartialEq<Self> for ThingType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.thing.eq(&other.thing)
    }
}

impl<T> Eq for ThingType<T> {}

impl<T> PartialOrd<Self> for ThingType<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for ThingType<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.thing.cmp(&other.thing)
    }
}

impl<T> Hash for ThingType<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.thing.hash(state)
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
            ________optional_data_______: None,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// some usefull functions for Thing
/// ```
/// #[derive(surrealdb_extras::SurrealTable, serde::Serialize, serde::Deserialize)]
/// #[db("test_table")]
/// struct Test {
///     name: String,
///     /// a refrence to another table entry
///     refr: surrealdb_extras::ThingFunc
/// }
/// ```
pub struct ThingFunc(pub Thing);

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

impl From<Thing> for ThingFunc {
    fn from(value: Thing) -> Self {
        ThingFunc::new(value)
    }
}

impl ThingFunc {
    /// From Thing
    pub fn new(thing: Thing) -> Self {
        Self(thing)
    }

    /// deletes from db and return value
    pub fn delete<T, C: Connection>(self, conn: &Surreal<C>) -> Delete<C, Option<T>> {
        conn.delete((self.0.tb, self.0.id))
    }

    /// gets from db
    pub fn get<T, C: Connection>(self, conn: &Surreal<C>) -> Select<C, Option<T>> {
        conn.select((self.0.tb, self.0.id))
    }

    /// Replaces the current document / record data with the specified data
    pub fn replace<T: DeserializeOwned, C: Connection, D: Serialize>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Content<C, D, Option<T>> {
        conn.update((self.0.tb, self.0.id)).content(data)
    }

    /// Merges the current document / record data with the specified data
    pub fn update<T: DeserializeOwned, C: Connection, D: Serialize>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Merge<C, D, Option<T>> {
        conn.update((self.0.tb, self.0.id)).merge(data)
    }

    /// Patches the current document / record data with the specified JSON Patch data
    pub fn patch<T: DeserializeOwned, C: Connection>(
        self,
        conn: &Surreal<C>,
        data: PatchOp,
    ) -> Patch<C, Option<T>> {
        conn.update((self.0.tb, self.0.id)).patch(data)
    }

    /// deletes from db and return success
    pub async fn delete_s<C: Connection>(self, conn: &Surreal<C>) -> Result<bool, Error> {
        let r: Option<Record> = conn.delete((self.0.tb, self.0.id)).await?;
        Ok(r.is_some())
    }

    /// gets part from db
    pub async fn get_part<C: Connection, T: SurrealSelectInfo>(
        self,
        conn: &Surreal<C>,
    ) -> Result<Option<RecordData<T>>, Error> {
        conn.query(format!(
            "SELECT {} FROM {} WHERE id = {}",
            T::keys().join(", "),
            self.tb(),
            self
        ))
        .await?
        .take(0)
    }

    /// returns table
    pub fn tb(&self) -> &String {
        &self.0.tb
    }

    /// returns id
    pub fn id(&self) -> &Id {
        &self.0.id
    }
}
