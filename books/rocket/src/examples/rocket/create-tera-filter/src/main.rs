#[macro_use]
extern crate rocket;

mod mytera;

#[cfg(test)]
mod tests;

use rocket::response::content::RawHtml;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"See <a href="/hello/Foo Bar">Foo Bar</a>."#)
}

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render(
        "index",
        context! {
            title: "Hello",
            name: Some(name),
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .attach(Template::custom(|engines| {
            mytera::customize(&mut engines.tera);
        }))
}
