#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub mod db;
use crate::db::Item;

#[derive(FromForm)]
struct AddForm<'r> {
    title: &'r str,
}

async fn form_and_list(dbh: &State<Surreal<Client>>) -> Template {
    let items: Vec<Item> = db::get_items(dbh).await.unwrap();
    for item in &items {
        rocket::info!("{} {}", item.id.id, item.title);
    }
    Template::render(
        "index",
        context! {
            title: "TODO",
            items,
        },
    )
}

#[get("/")]
async fn get_index(dbh: &State<Surreal<Client>>) -> Template {
    form_and_list(dbh).await
}

#[post("/", data = "<input>")]
async fn post_index(dbh: &State<Surreal<Client>>, input: Form<AddForm<'_>>) -> Template {
    rocket::info!("Add '{}'", input.title);
    let title = input.title.trim();
    db::add_item(dbh, title).await.unwrap();
    form_and_list(dbh).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_index, post_index,])
        .attach(Template::fairing())
        .attach(db::fairing())
}

