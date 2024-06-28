use std::borrow::Cow;
use serde::{Serialize, Deserialize};
//use serde_json::json;
use surrealdb::{Error, Surreal};
use surrealdb::engine::remote::ws::Ws;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    title: String,
    name: Name,
}

// Pro tip: Replace String with Cow<'static, str> to
// avoid unnecessary heap allocations when inserting

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}

// Install at https://surrealdb.com/install
// and use `surreal start --user root --pass root`
// to start a working database to take the following queries
// See the results via `surreal sql --ns namespace --db database --pretty`
// or https://surrealist.app/
// followed by the query `SELECT * FROM person;`
#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;

    db.query("DELETE person").await?;

    // Create a new person with a random ID
    let created: Vec<Person> = db.create("person")
        .content(Person {
            title: "Founder & CEO".into(),
            name: Name {
                first: "Tobie".into(),
                last: "Morgan Hitchcock".into(),
            },
        })
        .await?;

    // Select all people records
    let people: Vec<Person> = db.select("person").await?;
    dbg!(people);


    Ok(())
}
