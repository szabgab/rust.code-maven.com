use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::{RecordId, Surreal};

#[derive(Debug, Deserialize, Serialize)]
struct EntryNummer {
    id: RecordId,
    number: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct EntryWithName {
    id: RecordId,
    number: u32,
    name: Option<String>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;

    dbh.use_ns("demo").use_db("demo").await?;

    dbh.query("DEFINE TABLE entry SCHEMAFULL").await?;
    dbh.query("DEFINE FIELD number ON TABLE entry TYPE int")
        .await?;

    let res = dbh
        .query(
            "CREATE entry CONTENT {
        number: 42,

    };",
        )
        .await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");

    let mut entries = dbh.query("SELECT * FROM entry").await?;
    let entries: Vec<EntryNummer> = entries.take(0)?;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].number, 42);
    for entry in entries {
        println!("{} {}", entry.id, entry.number);
    }
    println!("---------");

    let res = dbh
        .query("DEFINE FIELD name ON TABLE entry TYPE string")
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
        name: 'twenty three',
    };",
        )
        .await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");

    let mut entries = dbh
        .query("SELECT * FROM entry ORDER BY number DESC")
        .await?;
    let entries: Vec<EntryWithName> = entries.take(0)?;
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].number, 42);
    assert_eq!(entries[0].name, None);
    assert_eq!(entries[1].number, 23);
    assert_eq!(entries[1].name, Some(String::from("twenty three")));
    for entry in entries {
        println!(
            "{} {} {}",
            entry.id,
            entry.number,
            entry.name.unwrap_or("missing".to_string())
        );
    }
    println!("---------");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::main;

    #[test]
    fn check() {
        main().unwrap();
    }
}
