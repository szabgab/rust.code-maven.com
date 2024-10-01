use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    id: Thing,
    number: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;

    dbh.use_ns("demo").use_db("demo").await?;

    dbh.query("DEFINE TABLE entry SCHEMAFULL").await?;
    dbh.query("DEFINE FIELD number ON TABLE entry TYPE int")
        .await?;
    dbh.query("DEFINE FIELD name ON TABLE entry TYPE string")
        .await?;
    dbh.query("DEFINE FIELD email ON TABLE entry TYPE string ASSERT string::is::email($value);")
        .await?;

    let res = dbh
        .query(
            "CREATE entry CONTENT {
        number: 42,
        name: 'Answer',
        email: 'foo@bar.com',

    };",
        )
        .await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");

    let res = dbh
        .query(
            "CREATE entry CONTENT {
        number: 19,
        name: 'NineTeen',
        email: 'not_an_email',
    };",
        )
        .await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");

    let res = dbh
        .query(
            "CREATE entry CONTENT {
        number: 23,
        email: 'missing@bar.com',
    };",
        )
        .await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");

    let mut entries = dbh.query("SELECT * FROM entry").await?;
    let entries: Vec<Entry> = entries.take(0)?;
    for entry in entries {
        println!(
            "{} {} {} {}",
            entry.id, entry.number, entry.name, entry.email
        );
    }
    println!("---------");

    Ok(())
}
