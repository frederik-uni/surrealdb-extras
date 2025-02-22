use crate::{RecordIdFunc, SurrealSelectInfo};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use surrealdb::method::{Content, Delete, Merge, Patch, Select};
use surrealdb::opt::PatchOp;
use surrealdb::{Connection, Error, RecordId, Surreal};

#[derive(Debug, Serialize, Deserialize)]
/// Deserialize response into id
pub struct Record {
    pub id: RecordIdFunc,
}

impl Record {
    /// From Thing
    pub fn new(id: RecordId) -> Self {
        Self {
            id: RecordIdFunc(id),
        }
    }

    /// deletes from db and return value
    pub fn delete<T, C: Connection>(self, conn: &Surreal<C>) -> Delete<C, Option<T>> {
        self.id.delete(conn)
    }

    /// deletes from db and return success
    pub async fn delete_s<C: Connection>(self, conn: &Surreal<C>) -> Result<bool, Error> {
        self.id.delete_s(conn).await
    }

    /// gets from db
    pub fn get<T, C: Connection>(self, conn: &Surreal<C>) -> Select<C, Option<T>> {
        self.id.get(conn)
    }

    /// Replaces the current document / record data with the specified data
    pub fn replace<T: DeserializeOwned, C: Connection, D: Serialize + 'static>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Content<C, Option<T>> {
        self.id.replace(conn, data)
    }

    /// Merges the current document / record data with the specified data
    pub fn merge<T: DeserializeOwned, C: Connection, D: Serialize>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Merge<C, D, Option<T>> {
        self.id.merge(conn, data)
    }

    /// Patches the current document / record data with the specified JSON Patch data
    pub fn patch<T: DeserializeOwned, C: Connection>(
        self,
        conn: &Surreal<C>,
        data: PatchOp,
    ) -> Patch<C, Option<T>> {
        self.id.patch(conn, data)
    }

    /// Gets part from db
    pub async fn get_part<C: Connection, T: SurrealSelectInfo>(
        self,
        conn: &Surreal<C>,
    ) -> Result<Option<RecordData<T>>, Error> {
        self.id.get_part(conn).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Deserialize response into id and data
pub struct RecordData<RD> {
    pub id: RecordIdFunc,
    #[serde(flatten)]
    pub data: RD,
}

impl<D> RecordData<D> {
    /// deletes from db and return value
    pub fn delete<T, C: Connection>(self, conn: &Surreal<C>) -> Delete<C, Option<T>> {
        self.id.delete(conn)
    }

    /// deletes from db and return success
    pub async fn delete_s<C: Connection>(self, conn: &Surreal<C>) -> Result<bool, Error> {
        self.id.delete_s(conn).await
    }

    /// gets from db
    pub fn get<T, C: Connection>(self, conn: &Surreal<C>) -> Select<C, Option<T>> {
        self.id.get(conn)
    }

    /// Replaces the current document / record data with the specified data
    pub fn replace<T: DeserializeOwned, C: Connection, ID: Serialize + 'static>(
        self,
        conn: &Surreal<C>,
        data: ID,
    ) -> Content<C, Option<T>> {
        self.id.replace(conn, data)
    }

    /// Merges the current document / record data with the specified data
    pub fn merge<T: DeserializeOwned, C: Connection, ID: Serialize>(
        self,
        conn: &Surreal<C>,
        data: ID,
    ) -> Merge<C, ID, Option<T>> {
        self.id.merge(conn, data)
    }

    /// Patches the current document / record data with the specified JSON Patch data
    pub fn patch<T: DeserializeOwned, C: Connection>(
        self,
        conn: &Surreal<C>,
        data: PatchOp,
    ) -> Patch<C, Option<T>> {
        self.id.patch(conn, data)
    }

    /// Gets part from db
    pub async fn get_part<C: Connection, T: SurrealSelectInfo>(
        self,
        conn: &Surreal<C>,
    ) -> Result<Option<RecordData<T>>, Error> {
        self.id.get_part(conn).await
    }
}
