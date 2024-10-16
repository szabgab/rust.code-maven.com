#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    rocket::trace!("Trace from Hello World");
    rocket::debug!("Debug from Hello World");
    rocket::info!("Info from Hello World");
    rocket::warn!("Warning from Hello World");
    rocket::error!("Error from Hello World");
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    rocket::build().mount("/", routes![index])
}
