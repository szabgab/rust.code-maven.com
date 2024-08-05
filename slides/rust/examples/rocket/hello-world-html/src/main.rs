#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml("Hello, <b>world!</b>")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod tests;
