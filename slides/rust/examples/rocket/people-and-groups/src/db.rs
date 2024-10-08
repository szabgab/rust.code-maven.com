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
const MEMBERSHIP: &str = "membership";

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
pub struct Membership {
    pub id: Thing,
    pub person: Thing,
    pub group: Thing,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MembershipWithPersonAndGroup {
    pub id: Thing,
    pub person: Person,
    pub group: Group,
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

        let query = format!(
            "DEFINE INDEX membership_ids ON TABLE {MEMBERSHIP} COLUMNS {PERSON}, {GROUP} UNIQUE"
        );
        dbh.query(query).await.unwrap();

        rocket.manage(dbh)
    })
}

pub async fn clear(dbh: &Surreal<Client>, database: String) -> surrealdb::Result<()> {
    rocket::info!("Clearing database");
    let result = dbh
        .query(format!("REMOVE DATABASE IF EXISTS `{database}`;"))
        .await?;
    result.check()?;

    Ok(())
}

pub async fn add_person(dbh: &Surreal<Client>, name: &str) -> surrealdb::Result<Person> {
    let entry = Person {
        id: Thing::from((PERSON, Id::ulid())),
        name: name.to_owned(),
    };

    dbh.create(Resource::from(PERSON))
        .content(entry.clone())
        .await?;

    Ok(entry)
}

pub async fn add_group(dbh: &Surreal<Client>, name: &str, uid: &str) -> surrealdb::Result<Group> {
    let entry = Group {
        id: Thing::from((GROUP, Id::ulid())),
        name: name.to_owned(),
        owner: Thing::from((PERSON, Id::from(uid))),
    };

    dbh.create(Resource::from(GROUP))
        .content(entry.clone())
        .await?;

    Ok(entry)
}

pub async fn add_member(dbh: &Surreal<Client>, uid: &str, gid: &str) -> surrealdb::Result<()> {
    let entry = Membership {
        id: Thing::from((MEMBERSHIP, Id::ulid())),
        person: Thing::from((PERSON, Id::from(uid))),
        group: Thing::from((GROUP, Id::from(gid))),
    };

    dbh.create(Resource::from(MEMBERSHIP))
        .content(entry.clone())
        .await?;

    Ok(())
}

pub async fn get_memberships(
    dbh: &Surreal<Client>,
) -> surrealdb::Result<Vec<MembershipWithPersonAndGroup>> {
    let mut response = dbh
        .query("SELECT * FROM type::table($table) FETCH person, group")
        .bind(("table", MEMBERSHIP))
        .await?;

    let entries: Vec<MembershipWithPersonAndGroup> = response.take(0)?;
    rocket::info!("Response: {:?}", entries);
    Ok(entries)
}

pub async fn delete_membership(dbh: &Surreal<Client>, id: &str) -> surrealdb::Result<()> {
    let res = dbh
        .query("DELETE type::table($table) WHERE id=type::thing($table, $id)")
        .bind(("table", MEMBERSHIP))
        .bind(("id", id.to_owned()))
        .await?;
    res.check()?;

    Ok(())
}

pub async fn get_people(dbh: &Surreal<Client>) -> surrealdb::Result<Vec<Person>> {
    let mut response = dbh
        .query(format!("SELECT * FROM type::table($table);"))
        .bind(("table", PERSON))
        .await?;
    let entries: Vec<Person> = response.take(0)?;
    Ok(entries)
}

pub async fn get_person(dbh: &Surreal<Client>, id: &str) -> surrealdb::Result<Option<Person>> {
    dbh.select((PERSON, id)).await
}

pub async fn get_person_with_groups(
    dbh: &Surreal<Client>,
    id: &str,
) -> surrealdb::Result<Option<(Person, Vec<Group>)>> {
    let person = get_person(dbh, id).await?;
    match person {
        None => return Ok(None),
        Some(person) => {
            let groups = get_groups_by_owner(dbh, id).await?;
            Ok(Some((person, groups)))
        }
    }
}

pub async fn get_groups_by_owner(
    dbh: &Surreal<Client>,
    uid: &str,
) -> surrealdb::Result<Vec<Group>> {
    rocket::info!("Getting groups for {}", uid);
    let mut response = dbh
        .query(
            "SELECT * FROM type::table($group_table) WHERE owner=type::thing($user_table, $uid);",
        )
        .bind(("group_table", GROUP))
        .bind(("uid", uid.to_owned()))
        .bind(("user_table", PERSON))
        .await?;
    let entries: Vec<Group> = response.take(0)?;
    Ok(entries)
}

pub async fn get_groups(dbh: &Surreal<Client>) -> surrealdb::Result<Vec<Group>> {
    let mut response = dbh.query(format!("SELECT * FROM {GROUP};")).await?;
    let entries: Vec<Group> = response.take(0)?;
    Ok(entries)
}

pub async fn get_group(dbh: &Surreal<Client>, id: &str) -> surrealdb::Result<Option<Group>> {
    dbh.select((GROUP, id)).await
}

pub async fn get_group_with_owner(
    dbh: &Surreal<Client>,
    id: &str,
) -> surrealdb::Result<Option<GroupWithOwner>> {
    let mut response = dbh
        .query("SELECT * FROM type::table($group_table) WHERE id=type::thing($id) FETCH owner")
        .bind(("group_table", GROUP))
        .bind(("id", format!("{GROUP}:{id}")))
        .await?;

    let entries: Vec<GroupWithOwner> = response.take(0)?;
    rocket::info!("Response: {:?}", entries);
    Ok(entries.get(0).cloned())
}
