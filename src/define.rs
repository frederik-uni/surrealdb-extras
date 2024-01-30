use crate::surreal_table::Register;
use std::collections::{HashMap, HashSet};
use surrealdb::sql::Value;
use surrealdb::{Connect, Connection, Surreal};

/// creates namespace, db, tables and defines the attributes if they do not exist
/// ```
/// use std::path::PathBuf;
/// use surrealdb::key::namespace::db::Db;
/// use surrealdb::opt::Config;
/// use surrealdb::Surreal;
/// pub async fn establish(path: PathBuf) -> surrealdb::Result<Surreal<Db>> {
///     let conn = Surreal::new::<Mem>((path.join("db"), Config::default().strict()));
///     surrealdb_extras::use_ns_db(conn, "test", "test", vec![Test::register]).await
/// }
/// ```
pub async fn use_ns_db<C: Connection>(
    conn: Connect<C, Surreal<C>>,
    namespace: &str,
    db: &str,
    register: Vec<Register>,
) -> surrealdb::Result<Surreal<C>> {
    let conn = conn.await?;
    if missing(&conn, "INFO FOR KV", ("namespaces", namespace)).await {
        conn.query(format!("DEFINE NAMESPACE {};", namespace))
            .await?;
    }
    conn.use_ns(namespace).await?;

    if missing(&conn, "INFO FOR NS", ("databases", db)).await {
        conn.query(format!("DEFINE DATABASE {};", db)).await?;
    }
    conn.use_db(db).await?;
    let mut hm = HashMap::new();
    for (name, path, _) in register.iter() {
        let name = name();
        let path = path();
        hm.insert(path, name);
    }

    let tables = table_list(&conn).await;
    for (name, _, funcs) in register {
        if tables.get(name()).is_none() {
            for query in funcs(&hm) {
                println!("{}", query);
                conn.query(query).await?;
            }
        }
    }
    Ok(conn)
}

async fn missing<C: Connection>(conn: &Surreal<C>, query: &str, key_value: (&str, &str)) -> bool {
    conn.query(query)
        .await
        .unwrap()
        .take::<Value>(0)
        .unwrap()
        .into_json()
        .as_object()
        .unwrap()
        .get_key_value(key_value.0)
        .unwrap()
        .1
        .clone()
        .as_object()
        .unwrap()
        .get_key_value(key_value.1)
        .is_none()
}

async fn table_list<C: Connection>(conn: &Surreal<C>) -> HashSet<String> {
    conn.query("INFO FOR DB")
        .await
        .unwrap()
        .take::<Value>(0)
        .unwrap()
        .into_json()
        .as_object()
        .unwrap()
        .get_key_value("tables")
        .unwrap()
        .1
        .clone()
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<HashSet<_>>()
}
