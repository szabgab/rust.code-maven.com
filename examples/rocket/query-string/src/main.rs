#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/show?<shape>&<color>")]
fn show(shape: String, color: String) -> Template {
    println!("shape: {} {}", shape, color);

    Template::render(
        "show",
        context! {
            shape: shape,
            color: color
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, show])
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;
