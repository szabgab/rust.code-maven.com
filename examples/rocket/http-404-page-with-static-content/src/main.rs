#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::response::content::RawHtml;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml("Hello, <b>world!</b>")
}

#[catch(404)]
fn not_found() -> RawHtml<&'static str> {
    const BODY: &str = include_str!("templates/404.html");
    RawHtml(BODY)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
}

#[cfg(test)]
mod tests;
