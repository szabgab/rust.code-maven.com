#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<String> {
    let html = String::from(
        r#"
    <a href="/user/42">User 42</a><br>
    <a href="/user/foo">User foo - not valid</a><br>
    "#,
    );

    content::RawHtml(html)
}

#[get("/user/<uid>")]
fn user(uid: usize) -> content::RawHtml<String> {
    let html = format!("User ID: {uid}");
    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, user])
}

#[cfg(test)]
mod tests;
