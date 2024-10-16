#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::http::Status;
use rocket::response::{content, Redirect};

#[derive(FromForm)]
struct LoginForm<'r> {
    username: &'r str,
    password: &'r str,
}

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
    <form method="POST" action="/login">
    Username: <input name="username"> Password: <input type="password" name="password"> <input type="submit" value="Login">
    </form>
    "#,
    )
}

#[post("/login", data = "<input>")]
fn login(input: Form<LoginForm>) -> std::result::Result<Redirect, Status> {
    if input.username == "foo" && input.password == "secret" {
        Ok(Redirect::to(uri!(home)))
    } else {
        //Outcome::Error(Status::BadRequest)
        Err(Status::BadRequest)
    }
}

#[get("/home")]
fn home() -> content::RawHtml<&'static str> {
    content::RawHtml("Home")
}

#[catch(400)]
fn http_400() -> content::RawHtml<&'static str> {
    content::RawHtml("This is a 400 error")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, login, home])
        .register("/", catchers![http_400])
}

#[cfg(test)]
mod tests;
