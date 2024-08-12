#[macro_use]
extern crate rocket;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use serde::Deserialize;

use rocket_dyn_templates::{context, Template};
use rocket::{fairing::AdHoc, State};


struct HitCount {
    count: AtomicUsize
}



#[derive(Deserialize)]
struct MyConfig {
    title: String,
}



#[get("/")]
fn index(hit_count: &State<HitCount>, config: &State<MyConfig>) -> Template {
    //let current_count = hit_count.count.load(Ordering::Relaxed);
    let current_count = hit_count.count.fetch_add(1, Ordering::Relaxed);

    Template::render("index", context! {
        title: &config.title,
        count: current_count,
    })
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", context! {
        // Currently the title is set in the template
        //title: "404 Not Found"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .attach(AdHoc::config::<MyConfig>())
        .manage(HitCount { count: AtomicUsize::new(0) })
        .register("/", catchers![not_found])

}

#[cfg(test)]
mod tests;

