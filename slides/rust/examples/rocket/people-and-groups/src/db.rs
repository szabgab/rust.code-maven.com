use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::opt::Resource;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

const PERSON: &str = "person";
const GROUP: &str = "group";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Person {
    pub id: Thing,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Group {
    id: Thing,
    name: String,
    owner: Thing,
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
        let db_namespace = "rust-slides";
        let db_database = "people-and-groups";

        let dbh = Surreal::new::<Ws>(address).await.unwrap();

        dbh.signin(Root { username, password }).await.unwrap();

        dbh.use_ns(db_namespace).use_db(db_database).await.unwrap();

        rocket.manage(dbh)
    })
}

pub async fn clear(dbh: &Surreal<Client>) -> surrealdb::Result<()> {
    dbh.query(format!("DELETE FROM {PERSON};")).await?;
    dbh.query(format!("DELETE FROM {GROUP};")).await?;
    Ok(())
}

pub async fn add_person(dbh: &Surreal<Client>, name: &str) -> surrealdb::Result<()> {
    let id = Id::ulid();

    let entry = Person {
        id: Thing::from(("items", id)),
        name: name.to_owned(),
    };

    dbh.create(Resource::from("items")).content(entry).await?;

    Ok(())
}

// pub async fn update_item(
//     dbh: &Surreal<Client>,
//     id: &str,
//     title: &str,
//     text: &str,
// ) -> surrealdb::Result<()> {
//     rocket::info!("Update '{}' '{}'", id, title);

//     #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
//     struct UpdateItem {
//         title: String,
//         text: String,
//     }

//     let _item: Option<Item> = dbh
//         .update(("items", id))
//         .merge(UpdateItem {
//             title: title.to_owned(),
//             text: text.to_owned(),
//         })
//         .await?;

//     Ok(())
// }

pub async fn get_people(dbh: &Surreal<Client>) -> surrealdb::Result<Vec<Person>> {
    let mut response = dbh.query(format!("SELECT * FROM {PERSON};")).await?;
    let entries: Vec<Person> = response.take(0)?;
    Ok(entries)
}


pub async fn get_person(dbh: &Surreal<Client>, id: &str) -> surrealdb::Result<Option<Person>> {
    dbh.select((PERSON, id)).await
}
