#[macro_use]
extern crate rocket;

use std::time::{SystemTime, UNIX_EPOCH};

use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<String> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let html = format!(
        "Hello, Rocket at {:?} seconds or {:?} nanoseconds since the epoch",
        since_the_epoch.as_secs(),
        since_the_epoch.as_nanos()
    );
    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
