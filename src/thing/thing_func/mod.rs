use crate::{Record, RecordData, SurrealSelectInfo};
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::method::{Content, Delete, Merge, Patch, Select};
use surrealdb::opt::PatchOp;
use surrealdb::{Connection, Error, RecordId, RecordIdKey, Surreal};

mod from;
mod r#impl;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
/// some usefull functions for Thing
/// ```
/// #[derive(surrealdb_extras::SurrealTable, serde::Serialize, serde::Deserialize)]
/// #[db("test_table")]
/// struct Test {
///     name: String,
///     /// a refrence to another table entry
///     refr: surrealdb_extras::RecordIdFunc
/// }
/// ```
pub struct RecordIdFunc(pub RecordId);

impl RecordIdFunc {
    /// From Thing
    pub fn new(thing: RecordId) -> Self {
        Self(thing)
    }

    /// deletes from db and return value
    pub fn delete<T, C: Connection>(self, conn: &Surreal<C>) -> Delete<C, Option<T>> {
        conn.delete(self.0)
    }

    /// gets from db
    pub fn get<T, C: Connection>(self, conn: &Surreal<C>) -> Select<C, Option<T>> {
        conn.select(self.0)
    }

    /// Replaces the current document / record data with the specified data
    pub fn replace<R: DeserializeOwned, C: Connection, D: Serialize + 'static>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Content<C, Option<R>> {
        conn.update(self.0).content(data)
    }

    /// Merges the current document / record data with the specified data
    pub fn merge<T: DeserializeOwned, C: Connection, D: Serialize>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Merge<C, D, Option<T>> {
        conn.update(self.0).merge(data)
    }

    /// Patches the current document / record data with the specified JSON Patch data
    pub fn patch<T: DeserializeOwned, C: Connection>(
        self,
        conn: &Surreal<C>,
        data: PatchOp,
    ) -> Patch<C, Option<T>> {
        conn.update(self.0).patch(data)
    }

    /// deletes from db and return success
    pub async fn delete_s<C: Connection>(self, conn: &Surreal<C>) -> Result<bool, Error> {
        let r: Option<Record> = conn.delete(self.0).await?;
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
    pub fn tb(&self) -> &str {
        self.0.table()
    }

    /// returns id
    pub fn id(&self) -> &RecordIdKey {
        self.0.key()
    }
}
