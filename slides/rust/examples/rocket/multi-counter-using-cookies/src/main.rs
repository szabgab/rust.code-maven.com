#[macro_use]
extern crate rocket;

use rocket::http::CookieJar;

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> String {
    let counter: u32 = match cookies.get("counter") {
        Some(cookie) => match cookie.value().parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!(
                    "Invalid value '{}' for the 'counter' cookie.",
                    cookie.value()
                );
                0
            }
        },
        None => 0,
    };
    let counter = counter + 1;
    cookies.add(("counter", counter.to_string()));

    format!("Counter: {}", counter)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod tests;
