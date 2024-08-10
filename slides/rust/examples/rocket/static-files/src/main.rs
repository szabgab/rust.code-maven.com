#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
        <link href="/css/style.css" rel="stylesheet">
        <script src="/js/demo.js"></script>
        Hello, <b>world!</b>
        "#,
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
}

#[cfg(test)]
mod tests;
