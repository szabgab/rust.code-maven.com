use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::opt::Resource;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

const NAMESPACE: &str = "rust-slides";
const PERSON: &str = "person";
const GROUP: &str = "group";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Person {
    pub id: Thing,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Group {
    pub id: Thing,
    pub name: String,
    pub owner: Thing,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GroupWithOwner {
    pub id: Thing,
    pub name: String,
    pub owner: Person,
}

/// # Panics
///
/// Panics when it fails to connect to the database
#[must_use]
pub fn fairing(database: String) -> AdHoc {
    AdHoc::on_ignite("Managed Database Connection", |rocket| async {
        let address = "127.0.0.1:8000";
        let username = "root";
        let password = "root";

        let dbh = Surreal::new::<Ws>(address).await.unwrap();

        dbh.signin(Root { username, password }).await.unwrap();

        dbh.use_ns(NAMESPACE).use_db(database).await.unwrap();

        rocket.manage(dbh)
    })
}

pub async fn clear(dbh: &Surreal<Client>, database: String) -> surrealdb::Result<()> {
    rocket::info!("Clearing database");
    let result = dbh.query(format!("REMOVE DATABASE `{database}`;")).await?;
    result.check()?;

    // dbh.query(format!("DELETE FROM {PERSON};")).await?;
    // dbh.query(format!("DELETE FROM {GROUP};")).await?;
    Ok(())
}

pub async fn add_person(dbh: &Surreal<Client>, name: &str) -> surrealdb::Result<Person> {
    let entry = Person {
        id: Thing::from((PERSON, Id::ulid())),
        name: name.to_owned(),
    };

    dbh.create(Resource::from(PERSON)).content(entry.clone()).await?;

    Ok(entry)
}

pub async fn add_group(dbh: &Surreal<Client>, name: &str, uid: &str) -> surrealdb::Result<()> {
    let entry = Group {
        id: Thing::from((GROUP, Id::ulid())),
        name: name.to_owned(),
        owner: Thing::from((PERSON, Id::from(uid))),
    };

    dbh.create(Resource::from(GROUP)).content(entry).await?;

    Ok(())
}

pub async fn get_people(dbh: &Surreal<Client>) -> surrealdb::Result<Vec<Person>> {
    let mut response = dbh.query(format!("SELECT * FROM {PERSON};")).await?;
    let entries: Vec<Person> = response.take(0)?;
    Ok(entries)
}

pub async fn get_person(dbh: &Surreal<Client>, id: &str) -> surrealdb::Result<Option<Person>> {
    dbh.select((PERSON, id)).await
}

pub async fn get_groups(dbh: &Surreal<Client>) -> surrealdb::Result<Vec<Group>> {
    let mut response = dbh.query(format!("SELECT * FROM {GROUP};")).await?;
    let entries: Vec<Group> = response.take(0)?;
    Ok(entries)
}

pub async fn get_group(dbh: &Surreal<Client>, id: &str) -> surrealdb::Result<Option<Group>> {
    dbh.select((GROUP, id)).await
}

// type::table($table)
pub async fn get_group_with_owner(
    dbh: &Surreal<Client>,
    id: &str,
) -> surrealdb::Result<Option<GroupWithOwner>> {
    let mut response = dbh
        .query(format!(
            "SELECT * FROM {GROUP} WHERE id=type::thing($id) FETCH owner"
        ))
        .bind(("id", format!("{GROUP}:{id}")))
        .await?;

    let entries: Vec<GroupWithOwner> = response.take(0)?;
    rocket::info!("Response: {:?}", entries);
    Ok(entries.get(0).cloned())
}
