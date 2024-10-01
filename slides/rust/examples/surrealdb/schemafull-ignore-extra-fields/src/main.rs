use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;


#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    id: Thing,
    number: u32,
}


#[derive(Debug, Deserialize, Serialize)]
struct EntryWithName {
    id: Thing,
    number: u32,
    name: Option<String>,
}


#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;

    dbh.use_ns("demo").use_db("demo").await?;

    dbh.query("DEFINE TABLE entry SCHEMAFULL").await?;
    dbh.query("DEFINE FIELD number ON TABLE entry TYPE int").await?;


    let res = dbh.query("CREATE entry CONTENT { number: 42 };").await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");

    let res = dbh.query("CREATE entry CONTENT { number: 19, name: 'NineTeen' };").await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");

    let mut entries = dbh.query("SELECT * FROM entry").await?;
    let entries: Vec<Entry> = entries.take(0)?;
    for entry in entries {
        println!("{} {}", entry.id, entry.number);
    }
    println!("---------");

    let mut entries = dbh.query("SELECT * FROM entry").await?;
    let entries: Vec<EntryWithName> = entries.take(0)?;
    for entry in entries {
        println!("{} {}  {}", entry.id, entry.number, entry.name.unwrap_or(String::from("NO NAME")));
    }
    println!("---------");

    Ok(())
}
