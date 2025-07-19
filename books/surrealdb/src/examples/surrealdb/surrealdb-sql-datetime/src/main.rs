use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::sql::{Datetime, Id, Thing};
use surrealdb::Surreal;

const EVENTS: &str = "events";

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: Thing,
    title: String,
    date: Datetime,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("namespace").use_db("database").await?;

    let _events: Vec<Event> = db
        .create(EVENTS)
        .content(Event {
            id: Thing::from((EVENTS, Id::rand())),
            title: String::from("Introduction to Rust"),
            date: Datetime::default(),
        })
        .await?;

    let events: Vec<Event> = db.select(EVENTS).await?;
    for event in events {
        println!("{event:#?}");
    }

    Ok(())
}
