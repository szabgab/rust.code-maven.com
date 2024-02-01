#[macro_use]
extern crate rocket;

use rocket::http::CookieJar;
use rocket::response::content;
use std::time::{SystemTime, UNIX_EPOCH};


fn get_time() -> String {
     let start = SystemTime::now();
     let since_the_epoch = start
     .duration_since(UNIX_EPOCH)
     .expect("Time went backwards");

     since_the_epoch.as_micros().to_string()
}

fn get_html(cookies: &CookieJar<'_>,  current_time: &str) -> content::RawHtml<String> {
    let saved_time: String = match cookies.get("cookie-demo") {
        Some(cookie) => cookie.value().to_owned(),
        None => String::from("No cookie"),
    };

    content::RawHtml(format!(r#"<a href="/">home</a> <a href="/set">set cookie</a> <a href="/delete">delete cookie</a><br>Current time: {current_time}<br>Saved time: {saved_time}<br>"#))
}

#[get("/")]
fn home(cookies: &CookieJar<'_>) -> content::RawHtml<String> {
    let current_time = get_time();
    rocket::info!("home current_time: {}", current_time);
    get_html(cookies, &current_time)
}

#[get("/set")]
fn set_cookie(cookies: &CookieJar<'_>) -> content::RawHtml<String> {
    let current_time = get_time();
    rocket::info!("set_cookie current_time: {}", current_time);
    cookies.add(("cookie-demo", current_time.clone()));
    get_html(cookies, &current_time)
}

#[get("/delete")]
fn delete_cookie(cookies: &CookieJar<'_>) -> content::RawHtml<String> {
    let current_time = get_time();
    rocket::info!("delete_cookie current_time: {}", current_time);
    cookies.remove("cookie-demo");
    get_html(cookies, &current_time)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home, delete_cookie, set_cookie])
}
