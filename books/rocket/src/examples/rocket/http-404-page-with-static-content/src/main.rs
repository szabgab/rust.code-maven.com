#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml("Hello, <b>world!</b>")
}

#[catch(404)]
fn not_found() -> content::RawHtml<&'static str> {
    const BODY: &str = include_str!("templates/404.html");
    content::RawHtml(BODY)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
}

#[cfg(test)]
mod tests;
