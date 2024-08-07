#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user/<id>?<msg>")]
fn user(id: u32, msg: &str) -> String {
    format!("id: {id} msg: {msg}")
}

#[get("/group/<id>?<msg>")]
fn group(id: u32, msg: Option<&str>) -> String {
    format!("id: {id} msg: {msg:?}")
}

#[get("/other")]
fn other() -> Redirect {
    Redirect::to(uri!(user(23, "hello")))
}

#[get("/redir-value")]
fn redir_value() -> Redirect {
    Redirect::to(uri!(group(42, Some("message"))))
}

#[get("/redir-none")]
fn redir_none() -> Redirect {
    // type annotations needed for `std::option::Option<_>`
    //Redirect::to(uri!(group(42, None)))

    let msg: Option<&str> = None;
    Redirect::to(uri!(group(42, msg)))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, user, other, group, redir_value, redir_none],
    )
}

#[cfg(test)]
mod tests;
