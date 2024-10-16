#[macro_use]
extern crate rocket;

// #[cfg(test)] mod tests;

use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

struct GoodGuard;
struct BadGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GoodGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        rocket::info!("from_request in GoodGuard");
        rocket::info!("client_ip: {:?}", req.client_ip());
        rocket::info!("uri: {:?}", req.uri());

        Outcome::Success(Self)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BadGuard {
    type Error = ();

    async fn from_request(_req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        rocket::info!("from_request in BadGuard");
        Outcome::Error((Status::BadRequest, ()))
    }
}

#[get("/")]
fn index(_g: GoodGuard) -> &'static str {
    "Hello, world!"
}

#[get("/good")]
fn good(_g: GoodGuard) -> &'static str {
    "Hello, world!"
}

#[get("/bad")]
fn bad(_g: BadGuard) -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, good, bad])
}
