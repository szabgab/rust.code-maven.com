#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket::fs::relative;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::content;

struct MyGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MyGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        rocket::info!("from_request");
        rocket::info!("ip: {:?}", req.real_ip());
        let slug: std::path::PathBuf = req.segments(1..).unwrap();
        rocket::info!("path: {:?}", slug);
        let mut filepath = std::path::PathBuf::from(relative!("pages")).join(slug);
        filepath.set_extension("md");
        rocket::info!("filepath: {:?}", filepath);

        if filepath.exists() {
            Outcome::Success(MyGuard)
        } else {
            Outcome::Error((Status::NotFound, ()))
        }
    }
}

#[get("/")]
fn index() -> content::RawHtml<String> {
    let html = String::from(
        r#"
    <a href="/blog/main">main</a><br>
    <a href="/blog/missing">missing</a><br>
    "#,
    );
    content::RawHtml(html)
}

#[get("/blog/<slug>")]
fn blog(slug: &str, _g: MyGuard) -> content::RawHtml<String> {
    let mut filepath = std::path::PathBuf::from(relative!("pages")).join(slug);
    filepath.set_extension("md");
    let content = std::fs::read_to_string(filepath).unwrap();

    let html = format!("slug: {:?} content: {}", slug, content);

    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, blog])
}
