use chrono::{DateTime, Utc};
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::opt::Resource;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Item {
    pub id: Thing,
    pub title: String,
    pub date: DateTime<Utc>,
}

/// # Panics
///
/// Panics when it fails to connect to the database
#[must_use]
pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("Managed Database Connection", |rocket| async {
        let address = "127.0.0.1:8000";
        let username = "root";
        let password = "root";
        let db_namespace = "todo";
        let db_database = "todo";

        let dbh = Surreal::new::<Ws>(address).await.unwrap();

        dbh.signin(Root { username, password }).await.unwrap();

        dbh.use_ns(db_namespace).use_db(db_database).await.unwrap();

        rocket.manage(dbh)
    })
}

pub async fn add_item(dbh: &Surreal<Client>, title: &str) -> surrealdb::Result<()> {
    let utc: DateTime<Utc> = Utc::now();

    let id = Id::ulid();

    let entry = Item {
        id: Thing::from(("items", id)),
        title: title.to_owned(),
        date: utc,
    };

    dbh.create(Resource::from("items")).content(entry).await?;

    Ok(())
}

pub async fn get_items(dbh: &Surreal<Client>) -> surrealdb::Result<Vec<Item>> {
    let mut response = dbh.query("SELECT * FROM items;").await?;
    let entries: Vec<Item> = response.take(0)?;
    Ok(entries)
}

pub async fn get_item(dbh: &Surreal<Client>, id: &str) -> surrealdb::Result<Option<Item>> {
    rocket::info!("get_item({})", id);

    dbh.select(("items", id)).await
        //.query("SELECT * FROM items WHERE id=items:01J8WDNEVWEPTPJFXQJ25M7J81;")
        //.bind(("id", id.to_owned()))
        //.await?;
//    let entries: Vec<Item> = response.take(0)?;
    // if let Some(entry) = entries.get(0) {
    //     rocket::info!("matched");
    //     Ok(Some(entry.clone()))
    // } else {
    //     Ok(None)
    // }
    //response
}
