#[macro_use]
extern crate rocket;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use rocket::response::content;
use rocket::State;

struct HitCount {
    count: AtomicUsize
}

#[get("/")]
fn index(hit_count: &State<HitCount>) -> content::RawHtml<String> {
    //let current_count = hit_count.count.load(Ordering::Relaxed);
    let current_count = hit_count.count.fetch_add(1, Ordering::Relaxed);

    content::RawHtml(format!("Rocket Counter: {current_count}"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .manage(HitCount { count: AtomicUsize::new(0) })
}

#[cfg(test)]
mod tests;

