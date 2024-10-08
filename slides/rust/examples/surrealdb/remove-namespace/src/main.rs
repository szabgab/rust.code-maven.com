use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    name: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;
    let namespace = "name-space";
    let database = "data-base";

    dbh.use_ns(namespace).use_db(database).await?;

    entries("Before", &dbh).await;

    let response = dbh
        .query("INSERT INTO people (name) VALUES ($name);")
        .bind(("name", "Foo"))
        .await?;
    response.check()?;

    let response = dbh
        .query("INSERT INTO people (name) VALUES ($name);")
        .bind(("name", "Bar"))
        .await?;
    response.check()?;

    entries("Data added", &dbh).await;

    let result = dbh
        .query(format!("REMOVE NAMESPACE `{namespace}`;"))
        .await?;
    result.check()?;
    //println!("{:?}", result);

    entries("After remove", &dbh).await;

    Ok(())
}

async fn entries(text: &str, dbh: &Surreal<Db>) {
    let mut rows = dbh.query("SELECT name FROM people").await.unwrap();
    let people: Vec<Entry> = rows.take(0).unwrap();
    println!("---- {text}: {}", people.len());
    for person in people {
        println!("{}", person.name);
    }
}
