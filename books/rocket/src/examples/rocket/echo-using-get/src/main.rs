#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/echo?<text>")]
fn echo(text: String) -> Template {
    println!("text: {:?}", text);

    Template::render(
        "echo",
        context! {
            text: text
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, echo])
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;
