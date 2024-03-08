mod from;
mod r#impl;

use crate::{RecordData, SurrealSelectInfo, SurrealTableInfo, ThingFunc};
use serde::Serialize;
use std::hash::Hash;
use std::marker::PhantomData;
use surrealdb::method::{Content, Delete, Merge, Patch};
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ThingType<T> {
    /// thing func
    pub thing: ThingFunc,
    /// should never be initialized
    parse_to: PhantomData<T>,
}

impl<T: SurrealTableInfo + SurrealSelectInfo> ThingType<T> {
    pub fn new(thing_func: ThingFunc) -> Self {
        Self {
            thing: thing_func,
            parse_to: Default::default(),
        }
    }
    pub fn new_thing(thing: Thing) -> Self {
        Self {
            thing: ThingFunc::new(thing),
            parse_to: Default::default(),
        }
    }
    pub async fn get_part<C: Connection, TT: SurrealSelectInfo>(
        self,
        conn: &Surreal<C>,
    ) -> Result<Option<RecordData<TT>>, Error> {
        self.thing.get_part(conn).await
    }

    pub async fn get<C: Connection>(
        self,
        conn: &Surreal<C>,
    ) -> Result<Option<RecordData<T>>, Error> {
        self.thing.get_part(conn).await
    }

    /// returns table
    pub fn tb(&self) -> &String {
        &self.thing.0.tb
    }

    /// returns id
    pub fn id(&self) -> &Id {
        &self.thing.0.id
    }
    /// deletes from db and return success
    pub async fn delete_s<C: Connection>(self, conn: &Surreal<C>) -> Result<bool, Error> {
        self.thing.delete_s(conn).await
    }

    /// Patches the current document / record data with the specified JSON Patch data
    pub fn patch<C: Connection>(self, conn: &Surreal<C>, data: PatchOp) -> Patch<C, Option<T>> {
        self.thing.patch(conn, data)
    }

    /// Merges the current document / record data with the specified data
    pub fn update<C: Connection, D: Serialize>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Merge<C, D, Option<T>> {
        self.thing.update(conn, data)
    }

    /// Replaces the current document / record data with the specified data
    pub fn replace<C: Connection, D: Serialize>(
        self,
        conn: &Surreal<C>,
        data: D,
    ) -> Content<C, D, Option<T>> {
        self.thing.replace(conn, data)
    }

    /// deletes from db and return value
    pub fn delete<C: Connection>(self, conn: &Surreal<C>) -> Delete<C, Option<T>> {
        self.thing.delete(conn)
    }
}
