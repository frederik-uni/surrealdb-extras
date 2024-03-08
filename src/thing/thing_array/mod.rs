use crate::{Record, RecordData, SurrealSelectInfo};
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::method::{Content, Delete, Merge, Patch, Select};
use surrealdb::opt::{PatchOp, Resource};
use surrealdb::sql::{Array, Thing, Value};
use surrealdb::{opt, Connection, Error, Surreal};

mod from;

pub struct ThingArray(pub Vec<Thing>);

impl<R> opt::IntoResource<Vec<R>> for ThingArray {
    fn into_resource(self) -> surrealdb::Result<Resource> {
        Ok(Resource::Array(Array::from(
            self.0.into_iter().map(Value::from).collect::<Vec<_>>(),
        )))
    }
}

impl ThingArray {
    /// deletes from db and return value
    pub fn delete<T, C: Connection>(self, conn: &Surreal<C>) -> Delete<C, Vec<T>> {
        conn.delete(self)
    }

    /// gets from db
    pub fn get<T, C: Connection>(self, conn: &Surreal<C>) -> Select<C, Vec<T>> {
        conn.select(self)
    }

    /// Replaces the current document / record data with the specified data
    pub fn replace<T: DeserializeOwned, C: Connection, D: Serialize>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Content<C, D, Vec<T>> {
        conn.update(self).content(data)
    }

    /// Merges the current document / record data with the specified data
    pub fn update<T: DeserializeOwned, C: Connection, D: Serialize>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Merge<C, D, Vec<T>> {
        conn.update(self).merge(data)
    }

    /// Patches the current document / record data with the specified JSON Patch data
    pub fn patch<T: DeserializeOwned, C: Connection>(
        self,
        conn: &Surreal<C>,
        data: PatchOp,
    ) -> Patch<C, Vec<T>> {
        conn.update(self).patch(data)
    }

    /// deletes from db and return success
    pub async fn delete_s<C: Connection>(self, conn: &Surreal<C>) -> Result<bool, Error> {
        let r: Vec<Record> = conn.delete(self).await?;
        Ok(!r.is_empty())
    }

    /// gets part from db
    pub async fn get_part<C: Connection, T: SurrealSelectInfo>(
        self,
        conn: &Surreal<C>,
    ) -> Result<Vec<RecordData<T>>, Error> {
        conn.query(format!("SELECT {} FROM {}", T::keys().join(", "), self))
            .await?
            .take(0)
    }
}
