#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"<a href="/language/rust">rust</a> <a href="/language/python">python</a>"#)
}

#[get("/language/<answer>")]
fn question(answer: &str) -> std::result::Result<content::RawHtml<&'static str>, Status> {
    if answer == "rust" {
        Ok(content::RawHtml("correct"))
    } else {
        Err(Status::BadRequest)
    }
}

#[catch(400)]
fn http_400() -> content::RawHtml<&'static str> {
    content::RawHtml("This is a 400 error")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, question])
        .register("/", catchers![http_400])
}

#[cfg(test)]
mod tests;
