use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![index]
}


#[get("/")]
pub fn index() -> &'static str {
    "Blog"
}


#[cfg(test)]
mod blog_tests;
