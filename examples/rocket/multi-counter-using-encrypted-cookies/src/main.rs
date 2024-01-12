#[macro_use]
extern crate rocket;

use rocket::http::CookieJar;

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> String {
    eprintln!("before");
    let counter: u32 = match cookies.get_private("counter") {
        Some(cookie) => match cookie.value().parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid value {} for the 'counter' cookie.", cookie.value());
                0
            },
        },
        None => {
            eprintln!("None");
            0
        },
    };
    let counter = counter + 1;
    cookies.add_private(("counter", counter.to_string()));

    format!("Counter: {}", counter)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod tests;
