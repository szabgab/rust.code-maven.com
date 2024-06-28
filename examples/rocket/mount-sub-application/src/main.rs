#[macro_use]
extern crate rocket;

pub(crate) mod admin;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/admin", admin::routes())
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_admin;