#[macro_use]
extern crate rocket;

use rocket::{fairing::AdHoc, State};
use serde::Deserialize;

#[derive(Deserialize)]
struct MyConfig {
    #[serde(default = "get_default_custom_in_default_section")]
    custom_in_default: String,

    #[serde(default = "get_default_custom_a")]
    custom_a: String,

    #[serde(default = "get_default_custom_b")]
    custom_b: String,
}

fn get_default_custom_in_default_section() -> String {
    String::from("some other default")
}

fn get_default_custom_a() -> String {
    String::from("some default for a")
}

fn get_default_custom_b() -> String {
    String::from("some default for b")
}

#[get("/")]
fn index(config: &State<MyConfig>) -> &'static str {
    rocket::info!(
        "profile is debug: {:?}",
        rocket::Config::default().profile == "debug"
    );

    rocket::info!("custom_a {:?}", config.custom_a);

    rocket::info!("custom_b {:?}", config.custom_b);

    rocket::info!("custom_in_default {:?}", config.custom_in_default);

    "See the console"
}

#[get("/bad")]
fn bad() -> &'static str {
    rocket::info!(
        "profile is debug: {:?}",
        rocket::Config::default().profile == "debug"
    );

    let custom_a: String = rocket::Config::figment()
        .extract_inner("custom_a")
        .unwrap_or(String::from("some default in a"));
    rocket::info!("custom_a {:?}", custom_a);

    let custom_b = rocket::Config::figment()
        .extract_inner::<String>("custom_b")
        .unwrap_or(String::from("some default in b"));
    rocket::info!("custom_b {:?}", custom_b);

    let custom_in_default: String = rocket::Config::figment()
        .extract_inner("custom_in_default")
        .unwrap_or(String::from("some other default"));
    rocket::info!("custom_in_default {:?}", custom_in_default);

    "See the console"
}

#[get("/defaults")]
fn defaults() -> &'static str {
    rocket::info!("default: {:#?}", rocket::Config::default());

    "See the console"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, bad, defaults])
        .attach(AdHoc::config::<MyConfig>())
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn home() {
        let client = Client::tracked(super::rocket()).unwrap();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}
