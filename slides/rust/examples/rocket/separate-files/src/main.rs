#[macro_use]
extern crate rocket;

pub(crate) mod blog;


#[get("/")]
fn index() -> &'static str {
    "Main page"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/blog", blog::routes())
}

#[cfg(test)]
mod tests;
