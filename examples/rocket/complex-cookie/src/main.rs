#[macro_use]
extern crate rocket;

use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};
use std::time::{SystemTime, UNIX_EPOCH};

struct MyCookie {
    animal: String,
    timestamp: u128
}

fn get_time() -> u128 {
     let start = SystemTime::now();
     let since_the_epoch = start
     .duration_since(UNIX_EPOCH)
     .expect("Time went backwards");

     since_the_epoch.as_micros()
}

fn get_html(cookies: &CookieJar<'_>) -> Template {
    let saved_time: String = match cookies.get("cookie-demo") {
        Some(cookie) => cookie.value().to_owned(),
        None => String::from("No cookie"),
    };

    Template::render("main", context! {
        //name: "Rocket with Tera"
    })
}

#[get("/")]
fn home(cookies: &CookieJar<'_>) -> Template {
    get_html(cookies)

    // let current_time = get_time();
    // rocket::info!("home current_time: {}", current_time);
    // get_html(cookies, &current_time)
}

// #[get("/set")]
// fn set_cookie(cookies: &CookieJar<'_>) -> content::RawHtml<String> {
//     let current_time = get_time();
//     rocket::info!("set_cookie current_time: {}", current_time);
//     cookies.add(("cookie-demo", current_time.clone()));
//     get_html(cookies, &current_time)
// }

// #[get("/delete")]
// fn delete_cookie(cookies: &CookieJar<'_>) -> content::RawHtml<String> {
//     let current_time = get_time();
//     rocket::info!("delete_cookie current_time: {}", current_time);
//     cookies.remove("cookie-demo");
//     get_html(cookies, &current_time)
// }


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home]).attach(Template::fairing())
}
