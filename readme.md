A simple library that establishes a connection and sets/creates the namespace&db and sets the types of the attributes

## Example
```rs
pub async fn establish(path: PathBuf) -> surrealdb::Result<Surreal<Db>> {
    let conn = Surreal::new::<SpeeDb>((path.join("db"), Config::default().strict()));
    surrealdb_extras::use_ns_db(conn, "test", "test", vec![Test::register]).await
}

#[derive(surrealdb_extras::SurrealTableEntry, Deserialize, Serialize, Clone, Default)]
#[db("test_table")]
#[sql(["DEFINE EVENT test_table_updated ON TABLE test_table WHEN $event = \"UPDATE\" AND $before.updated == $after.updated THEN (UPDATE $after.id SET updated = time::now() );"])]
struct Test {
    random_number: i32,
    /// renamed field
    #[opt(rename = "new_name")]
    #[serde(rename = "new_name")]
    data: String,
    /// overwrites the detected db type
    #[opt(db_type = "string")]
    data: CustomStructWithSerialize,
    /// will be excluded in get_or_insert check
    #[opt(exclude = true)]
    updated: Datetime
}

pub async fn demo() {
    let conn = establish(PathBuf::new()).await.unwrap();
    let test = Test::default();

    // creates new item with custom id and return self
    let _: Option<Test> = test
        .clone()
        .insert(&conn, Some(sql::Id::rand()))
        .await
        .unwrap();
        
    // creates new item with random id and return record
    let v: surrealdb_extras::Record = test.add(&conn).await.unwrap().unwrap();
    
    // delete record
    let v: Option<surrealdb_extras::Record> = v.delete(&conn).await.unwrap();
}
```

## usefull functions in:
- ThingFunc
- SurrealTableInfo

the functions in ThingType, RecordData, Record are from ThingFunc


## Categorized

##### init: 
- `use_ns_db`
- `impl SurrealTableInfo`(use `#[derive(SurrealTable, Serialize, Deserialize)]`)

##### Deserialize: 
- `impl SurrealSelectInfo`(use `#[derive(SurrealSelect, Deserialize)]` or `#[derive(SurrealTable, Serialize, Deserialize)]`) 
- `Record`
- `RecordData`
- `ThingFunc`(within structs)
- `ThingType`(within structs)

##### Serialize: 
- `impl SurrealTableInfo`(use `#[derive(SurrealTable, Serialize, Deserialize)]`)
- `ThingFunc`(within structs)
- `ThingType`(within structs)

- `ThingArray`