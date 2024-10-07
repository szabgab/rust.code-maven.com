#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

struct Config {
    database: String,
}
pub mod db;
//use crate::db::{Person, Group};

#[derive(FromForm)]
struct AddPerson<'r> {
    name: &'r str,
}

#[derive(FromForm)]
struct AddGroup<'r> {
    name: &'r str,
    uid: &'r str,
}

async fn index_page(dbh: &State<Surreal<Client>>) -> Template {
    let people = db::get_people(dbh).await.unwrap();
    let groups = db::get_groups(dbh).await.unwrap();
    Template::render(
        "index",
        context! {
            title: "People and Groups",
            people: people,
            groups: groups,
        },
    )
}

async fn list_people(dbh: &State<Surreal<Client>>) -> Template {
    let people = db::get_people(dbh).await.unwrap();
    rocket::info!("People: {:?}", people);

    let pairs = people
        .iter()
        .map(|item| {
            let id = item.id.id.to_string();
            (id, item)
        })
        .collect::<Vec<_>>();

    Template::render(
        "people",
        context! {
            title: "People",
            people: pairs,
        },
    )
}

async fn list_groups(dbh: &State<Surreal<Client>>) -> Template {
    let groups = db::get_groups(dbh).await.unwrap();
    rocket::info!("Groups: {:?}", groups);

    let pairs = groups
        .iter()
        .map(|item| {
            let id = item.id.id.to_string();
            (id, item)
        })
        .collect::<Vec<_>>();

    Template::render(
        "groups",
        context! {
            title: "Groups",
            groups: pairs,
        },
    )
}

// ----------------------------------

#[get("/")]
async fn get_index(dbh: &State<Surreal<Client>>) -> Template {
    index_page(&dbh).await
}

#[get("/clear")]
async fn clear_db(dbh: &State<Surreal<Client>>, config: &State<Config>) -> Template {
    rocket::info!("Clearing database {}", config.database);
    db::clear(&dbh, config.database.clone()).await.unwrap();
    index_page(&dbh).await
}

#[get("/people")]
async fn get_people(dbh: &State<Surreal<Client>>) -> Template {
    list_people(dbh).await
}

#[post("/add-person", data = "<input>")]
async fn post_add_person(dbh: &State<Surreal<Client>>, input: Form<AddPerson<'_>>) -> Template {
    let name = input.name.trim();
    rocket::info!("Add  person called '{name}'");
    db::add_person(dbh, name).await.unwrap();

    list_people(dbh).await
}

#[get("/person/<id>")]
async fn get_person(dbh: &State<Surreal<Client>>, id: String) -> Option<Template> {
    if let Some(person) = db::get_person(dbh, &id).await.unwrap() {
        return Some(Template::render(
            "person",
            context! {
                title: person.name.clone(),
                id: person.id.clone().id.to_string(),
                person: person,
            },
        ));
    }

    None
}

#[get("/groups")]
async fn get_groups(dbh: &State<Surreal<Client>>) -> Template {
    list_groups(dbh).await
}

#[get("/add-group?<uid>")]
async fn get_add_group(_dbh: &State<Surreal<Client>>, uid: String) -> Template {
    Template::render(
        "add_group",
        context! {
            title: "Add Group",
            uid: uid.to_string(),
        },
    )
}

#[post("/add-group", data = "<input>")]
async fn post_add_group(dbh: &State<Surreal<Client>>, input: Form<AddGroup<'_>>) -> Template {
    let name = input.name.trim();
    let uid = input.uid.trim();
    rocket::info!("Add  group called '{name}' to user '{uid}'");
    db::add_group(dbh, name, uid).await.unwrap();

    list_groups(dbh).await
}

#[get("/group/<id>")]
async fn get_group(dbh: &State<Surreal<Client>>, id: String) -> Option<Template> {
    if let Some(group) = db::get_group_with_owner(dbh, &id).await.unwrap() {
        rocket::info!("Group: {}", group.owner.id);
        return Some(Template::render(
            "group",
            context! {
                title: group.name.clone(),
                id: group.id.clone().id.to_string(),
                group: group,
            },
        ));
    }

    None
}

#[launch]
fn rocket() -> _ {
    start(String::from("people-and-groups"))
}

fn start(database: String) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                clear_db,
                get_index,
                get_people,
                get_person,
                post_add_person,
                get_groups,
                get_group,
                get_add_group,
                post_add_group,
            ],
        )
        .manage(Config {
            database: database.clone(),
        })
        .attach(Template::fairing())
        .attach(db::fairing(database))
}

#[cfg(test)]
mod tests {
    use super::start;

    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn test_main() {
        let client = Client::tracked(start(String::from("test-people-and-groups"))).unwrap();

        let response = client.get("/clear").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"<title>People and Groups</title>"#));
        assert!(html.contains(r#"<div>Number of people: 0</div>"#));
        assert!(html.contains(r#"<div>Number of groups: 0</div>"#));

        let response = client
            .post("/add-person")
            .header(ContentType::Form)
            .body("name=John")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client
            .post("/add-person")
            .header(ContentType::Form)
            .body("name=Mary Ann")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let html = response.into_string().unwrap();
        assert!(html.contains(r#">John</a></li>"#));
        assert!(html.contains(r#">Mary Ann</a></li>"#));

        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"<title>People and Groups</title>"#));
        assert!(html.contains(r#"<div>Number of people: 2</div>"#));
        assert!(html.contains(r#"<div>Number of groups: 0</div>"#));
    }
}
