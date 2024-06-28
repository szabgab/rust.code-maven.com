#[macro_use]
extern crate rocket;

use rocket::response::content;

struct Guess {
    guess: 
}

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
    <form action="/check">
    <input name="guess">
    <input type="submit" value="Try">
    "#)
}

#[get("/check?<guess>")]
fn check(guess: u32) -> content::RawHtml<String> {
    content::RawHtml(format!("The guess was: {guess}"))
}

#[catch(422)]
fn not_processed() -> content::RawHtml<&'static str> {
    content::RawHtml("This is a 422 errror")
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, check])
    .register("/", catchers![not_processed])
}

#[cfg(test)]
mod tests;

