use std::borrow::Cow;
use std::fmt;
use std::marker::PhantomData;

use crate::{RecordIdFunc, SurrealSelectInfo};
use serde::de::{self, value, DeserializeOwned, DeserializeSeed, MapAccess, Visitor};
use serde::ser::{self, SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use surrealdb::method::{Content, Delete, Merge, Patch, Select};
use surrealdb::opt::PatchOp;
use surrealdb::{Connection, Error, RecordId, Surreal};

#[derive(Debug, Serialize, Deserialize)]
/// Deserialize response into id
pub struct Record {
    pub id: RecordIdFunc,
}

impl<'de, RD> Deserialize<'de> for RecordData<RD>
where
    RD: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RecordDataVisitor<RD>(PhantomData<RD>);

        impl<'de, RD> Visitor<'de> for RecordDataVisitor<RD>
        where
            RD: Deserialize<'de>,
        {
            type Value = RecordData<RD>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "a map/object containing `id` and the flattened RD fields"
                )
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut id: Option<RecordIdFunc> = None;

                let filter = FilterOutIdMapAccess {
                    inner: map,
                    id_out: &mut id,
                };

                let data = RD::deserialize(value::MapAccessDeserializer::new(filter))?;

                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                Ok(RecordData { id, data })
            }
        }

        deserializer.deserialize_map(RecordDataVisitor::<RD>(PhantomData))
    }
}

/// Seed that forces keys to deserialize as Cow<str> so we can match "id".
struct CowStrSeed;

impl<'de> DeserializeSeed<'de> for CowStrSeed {
    type Value = Cow<'de, str>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Cow::<str>::deserialize(deserializer)
    }
}

/// MapAccess wrapper that *removes* the "id" entry and stores it into `id_out`.
struct FilterOutIdMapAccess<'a, M> {
    inner: M,
    id_out: &'a mut Option<RecordIdFunc>,
}

impl<'de, 'a, M> MapAccess<'de> for FilterOutIdMapAccess<'a, M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        loop {
            let key: Option<Cow<'de, str>> = self.inner.next_key_seed(CowStrSeed)?;
            let Some(key) = key else {
                return Ok(None);
            };

            if key.as_ref() == "id" {
                if self.id_out.is_some() {
                    return Err(de::Error::duplicate_field("id"));
                }
                let v: RecordIdFunc = self.inner.next_value()?;
                *self.id_out = Some(v);
                continue; // keep scanning for the next real RD key
            }

            let k = match key {
                Cow::Borrowed(s) => {
                    seed.deserialize(value::BorrowedStrDeserializer::<M::Error>::new(s))?
                }
                Cow::Owned(s) => seed.deserialize(value::StringDeserializer::<M::Error>::new(s))?,
            };
            return Ok(Some(k));
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.inner.next_value_seed(seed)
    }
}

impl<RD> Serialize for RecordData<RD>
where
    RD: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;

        self.data.serialize(FlattenIntoMap { map: &mut map })?;

        map.end()
    }
}

/// Serializer that only supports struct/map and forwards entries into an existing SerializeMap.
struct FlattenIntoMap<'a, M> {
    map: &'a mut M,
}

impl<'a, M> Serializer for FlattenIntoMap<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();
    type Error = M::Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ForwardMap<'a, M>;
    type SerializeStruct = ForwardStruct<'a, M>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(ForwardMap { map: self.map })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(ForwardStruct { map: self.map })
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _v: &T) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _var: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _v: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _idx: u32,
        _var: &'static str,
        _v: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _var: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _var: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom(
            "RD must serialize as a map/struct to be flattened",
        ))
    }
}

struct ForwardMap<'a, M> {
    map: &'a mut M,
}

impl<'a, M> SerializeMap for ForwardMap<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_key<K: ?Sized + Serialize>(&mut self, key: &K) -> Result<(), Self::Error> {
        self.map.serialize_key(key)
    }

    fn serialize_value<V: ?Sized + Serialize>(&mut self, value: &V) -> Result<(), Self::Error> {
        self.map.serialize_value(value)
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        self.map.serialize_entry(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(()) // don't end the outer map
    }
}

struct ForwardStruct<'a, M> {
    map: &'a mut M,
}

impl<'a, M> SerializeStruct for ForwardStruct<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.map.serialize_entry(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(()) // don't end the outer map
    }
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

#[derive(Debug)]
/// Deserialize response into id and data
pub struct RecordData<RD> {
    pub id: RecordIdFunc,
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
