use crate::{Record, RecordData, SurrealSelectInfo};
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::method::{Content, Delete, Merge, Patch, Select};
use surrealdb::opt::PatchOp;
use surrealdb::sql::{Id, Thing};
use surrealdb::{Connection, Error, Surreal};

mod from;
mod r#impl;

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
        conn.query(format!("SELECT {} FROM {}", T::keys().join(", "), self))
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
