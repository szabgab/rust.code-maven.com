#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/page")]
fn page() -> &'static str {
    "A page"
}

#[get("/other")]
fn other() -> Redirect {
    Redirect::to(uri!(page))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, page, other])
}

#[cfg(test)]
mod tests;
