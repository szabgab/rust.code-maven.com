use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::Thing;
use surrealdb::Surreal;


#[derive(Debug, Deserialize, Serialize)]
struct EntryNummer {
    id: Thing,
    number: u32,
}


#[derive(Debug, Deserialize, Serialize)]
struct EntryWithName {
    id: Thing,
    number: u32,
    name: Option<String>,
}

// #[derive(Debug, Deserialize, Serialize)]
// struct NameOnly {
//     id: Thing,
//     name: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct NameEmail {
//     id: Thing,
//     name: String,
//     email: Option<String>,
// }


#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;

    dbh.use_ns("demo").use_db("demo").await?;

    dbh.query("DEFINE TABLE entry SCHEMAFULL").await?;
    dbh.query("DEFINE FIELD number ON TABLE entry TYPE int").await?;


    let res = dbh.query("CREATE entry CONTENT {
        number: 42,

    };").await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");


    let mut entries = dbh.query("SELECT * FROM entry").await?;
    let entries: Vec<EntryNummer> = entries.take(0)?;
    for entry in entries {
        println!("{} {}", entry.id, entry.number);
    }
    println!("---------");

    let res = dbh.query("DEFINE FIELD name ON TABLE entry TYPE string").await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");

    let res = dbh.query("CREATE entry CONTENT {
        number: 23,
        name: 'twenty three',
    };").await?;
    match res.check() {
        Ok(val) => println!("Success: {val:?}"),
        Err(err) => println!("Error: {err}"),
    }
    println!("---------");


    let mut entries = dbh.query("SELECT * FROM entry").await?;
    let entries: Vec<EntryWithName> = entries.take(0)?;
    for entry in entries {
        println!("{} {} {}", entry.id, entry.number, entry.name.unwrap_or("missing".to_string()));
    }
    println!("---------");

    // let mut entries = dbh.query("SELECT * FROM entry").await?;
    // let entries: Vec<EntryWithName> = entries.take(0)?;
    // for entry in entries {
    //     println!("{} {}  {}", entry.id, entry.number, entry.name.unwrap_or(String::from("NO NAME")));
    // }
    // println!("---------");

    // dbh.query("DEFINE FIELD name ON TABLE entry TYPE string").await?;
    // dbh.query("DEFINE FIELD email ON TABLE user TYPE string ASSERT string::is::email($value);").await?;


    // dbh.query("CREATE entry CONTENT {
    //     name: 'First entry',
    // };").await?;
    
    // let mut res = dbh.query("CREATE entry CONTENT {
    //     name: 'Second entry',
    //     email: 'foobar.com',
    // };").await?;
    // let x = res.check().unwrap();
    // //let errors = res.take_errors();
    // //println!("{:?}", errors);

    // list_name(&dbh).await?;
    // list_name_email(&dbh).await?;

    Ok(())
}


// async fn list_name(dbh: &Surreal<Db>) -> surrealdb::Result<()> {
//     let mut entries = dbh.query("SELECT id, name FROM entry").await?;
//     let entries: Vec<NameOnly> = entries.take(0)?;
//     for entry in entries {
//         println!("{} {}", entry.id, entry.name);
//     }
//     println!("---------");
//     Ok(())
// }

// async fn list_name_email(dbh: &Surreal<Db>) -> surrealdb::Result<()> {
//     let mut entries = dbh.query("SELECT id, name, email FROM entry").await?;
//     let entries: Vec<NameEmail> = entries.take(0)?;
//     for entry in entries {
//         println!("{} {:15} {}", entry.id, entry.name, entry.email.unwrap_or("NO EMAIL".to_string()));
//     }
//     println!("---------");

//     Ok(())
// }
