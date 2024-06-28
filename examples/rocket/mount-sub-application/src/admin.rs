//#[macro_use]
//extern crate rocket;
use rocket::Route;


pub fn routes() -> Vec<Route> {
    routes![main, list]
}

#[get("/")]
fn main() -> &'static str {
    "Admin main page"
}

#[get("/list")]
fn list() -> &'static str {
    "Admin list page"
}
