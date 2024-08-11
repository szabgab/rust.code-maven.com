#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket::fs::relative;
use rocket::response::content;

use rocket::request::FromParam;
#[derive(Debug)]
struct MyPath {
    filepath: std::path::PathBuf,
}

impl<'r> FromParam<'r> for MyPath {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        rocket::info!("from_param: {:?}", param);
        let mut filepath = std::path::PathBuf::from(relative!("pages")).join(param);
        filepath.set_extension("md");
        rocket::info!("filepath: {:?}", filepath);

        if filepath.exists() {
            Ok(Self { filepath: filepath })
        } else {
            Err("bad")
        }
    }
}

#[get("/")]
fn index() -> content::RawHtml<String> {
    let html = format!(
        r#"
    <a href="/blog/main">main</a><br>
    <a href="/blog/missing">missing</a><br>
    "#
    );
    content::RawHtml(html)
}

#[get("/blog/<slug>")]
fn blog(slug: MyPath) -> content::RawHtml<String> {
    rocket::info!("slug: {:?}", slug);

    let content = std::fs::read_to_string(slug.filepath).unwrap();

    let html = format!("content: {}", content);

    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, blog])
}
