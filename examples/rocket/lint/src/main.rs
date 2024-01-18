#[macro_use]
extern crate rocket;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                hello_world
            ],
        )
}

// cargo clippy -- -Dclippy::no_effect_underscore_binding