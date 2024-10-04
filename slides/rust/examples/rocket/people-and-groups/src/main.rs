#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

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
async fn clear_db(dbh: &State<Surreal<Client>>) -> Template {
    db::clear(&dbh).await.unwrap();
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
        .attach(Template::fairing())
        .attach(db::fairing())
}
