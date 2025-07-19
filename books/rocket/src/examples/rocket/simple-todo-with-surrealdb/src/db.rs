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
    pub text: Option<String>,
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

pub async fn clear(dbh: &Surreal<Client>) -> surrealdb::Result<()> {
    dbh.query("DELETE FROM items;").await?;
    Ok(())
}

pub async fn add_item(dbh: &Surreal<Client>, title: &str) -> surrealdb::Result<()> {
    let utc: DateTime<Utc> = Utc::now();

    let id = Id::ulid();

    let entry = Item {
        id: Thing::from(("items", id)),
        title: title.to_owned(),
        text: None,
        date: utc,
    };

    dbh.create(Resource::from("items")).content(entry).await?;

    Ok(())
}

pub async fn update_item(
    dbh: &Surreal<Client>,
    id: &str,
    title: &str,
    text: &str,
) -> surrealdb::Result<()> {
    rocket::info!("Update '{}' '{}'", id, title);

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
    struct UpdateItem {
        title: String,
        text: String,
    }

    let _item: Option<Item> = dbh
        .update(("items", id))
        .merge(UpdateItem {
            title: title.to_owned(),
            text: text.to_owned(),
        })
        .await?;

    Ok(())
}

pub async fn get_items(dbh: &Surreal<Client>) -> surrealdb::Result<Vec<Item>> {
    let mut response = dbh.query("SELECT * FROM items;").await?;
    let entries: Vec<Item> = response.take(0)?;
    Ok(entries)
}

pub async fn get_item(dbh: &Surreal<Client>, id: &str) -> surrealdb::Result<Option<Item>> {
    dbh.select(("items", id)).await
}
