#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

// deep
#[get("/deep/<number>")]
fn deep_indentation(number: &str) -> Template {
    match number.parse::<i32>() {
        Ok(number) => {
            // lots of processing with number
            Template::render("number", context! { number })
        }
        Err(err) => Template::render(
            "message",
            context! { message: format!("not a number {err}") },
        ),
    }
}

// early return using match
#[get("/early-match/<number>")]
fn early_with_match(number: &str) -> Template {
    let number = match number.parse::<i32>() {
        Ok(number) => number,
        Err(err) => {
            return Template::render(
                "message",
                context! { message: format!("not a number {err}") },
            )
        }
    };

    // lots of processing with number
    Template::render("number", context! { number })
}

#[get("/early-map/<number>")]
fn early_with_map_err(number: &str) -> Result<Template, Template> {
    let number = number.parse::<i32>().map_err(|err| {
        Template::render(
            "message",
            context! { message: format!("not a number {err}") },
        )
    })?;

    // lots of processing with number
    Ok(Template::render("number", context! { number }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                deep_indentation,
                early_with_match,
                early_with_map_err
            ],
        )
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;
