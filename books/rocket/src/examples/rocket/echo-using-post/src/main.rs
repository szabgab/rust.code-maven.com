#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

#[derive(FromForm)]
struct InputText<'r> {
    text: &'r str,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[post("/echo", data = "<input>")]
fn echo(input: Form<InputText<'_>>) -> Template {
    println!("text: {:?}", input.text);

    Template::render(
        "echo",
        context! {
            text: input.text
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
