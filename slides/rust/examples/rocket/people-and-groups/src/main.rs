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

// #[derive(FromForm)]
// struct UpdateForm<'r> {
//     id: &'r str,
//     title: &'r str,
//     text: &'r str,
// }

// async fn list_people(dbh: &State<Surreal<Client>>) -> Template {
//     let items: Vec<Item> = db::get_items(dbh).await.unwrap();
//     for item in &items {
//         rocket::info!("{} {}", item.id.id, item.title);
//     }

//     let pairs = items
//         .iter()
//         .map(|item| {
//             let id = item.id.id.to_string();
//             (id, item)
//         })
//         .collect::<Vec<_>>();

//     Template::render(
//         "index",
//         context! {
//             title: "TODO",
//             items: pairs,
//         },
//     )
// }


async fn index_page() -> Template {
    Template::render(
        "index",
        context! {
            title: "People and Groups",
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

// ----------------------------------


#[get("/")]
async fn get_index(_dbh: &State<Surreal<Client>>) -> Template {
    index_page().await
}

#[get("/clear")]
async fn clear_db(dbh: &State<Surreal<Client>>) -> Template {
    db::clear(&dbh).await.unwrap();
    index_page().await
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

// #[get("/item/<id>")]
// async fn get_item(dbh: &State<Surreal<Client>>, id: String) -> Option<Template> {
//     if let Some(item) = db::get_item(dbh, &id).await.unwrap() {
//         return Some(Template::render(
//             "item",
//             context! {
//                 title: item.title.clone(),
//                 id: item.id.clone().id.to_string(),
//                 item: item,
//             },
//         ));
//     }

//     None
// }

// #[post("/update", data = "<input>")]
// async fn update_item(dbh: &State<Surreal<Client>>, input: Form<UpdateForm<'_>>) -> Template {
//     rocket::info!("Update '{}'", input.id);
//     let id = input.id.trim();
//     let title = input.title.trim();
//     let text = input.text.trim();
//     db::update_item(dbh, id, title, text).await.unwrap();

//     form_and_list(dbh).await
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![clear_db, get_index, get_people, post_add_person],
        )
        .attach(Template::fairing())
        .attach(db::fairing())
}
