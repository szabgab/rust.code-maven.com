#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::{fairing::AdHoc, State};
use serde::Deserialize;

#[derive(Deserialize)]
struct MyConfig {
    custom: String,
}

#[get("/")]
fn index(config: &State<MyConfig>) -> content::RawHtml<String> {
    content::RawHtml(format!("Custom: {}", config.custom))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(AdHoc::config::<MyConfig>())
}

#[cfg(test)]
mod test {
    use rocket::config::Config;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn home() {
        let client = Client::tracked(super::rocket()).unwrap();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert_eq!(html, "Custom: In Rocket.toml");
    }

    #[test]
    fn test_1() {
        let provider = Config::figment().merge(("custom", "In Test 1"));
        let app = super::rocket().configure(provider);

        let client = Client::tracked(app).unwrap();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert_eq!(html, "Custom: In Test 1");
    }


    #[test]
    fn test_2() {
        let provider = Config::figment().merge(("custom", "In Test 2"));
        let app = super::rocket().configure(provider);

        let client = Client::tracked(app).unwrap();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert_eq!(html, "Custom: In Test 2");
    }

}
