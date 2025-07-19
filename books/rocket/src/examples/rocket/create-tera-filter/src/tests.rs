use super::rocket;

use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_name() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/hello/Tera%20Rocket").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let html = response.into_string().unwrap();
    assert!(html.contains("Hi Tera Rocket!"));
    assert!(html.contains("The name is 11 characters long."));
    assert!(html.contains("My number is 11."));
}

#[test]
fn test_index() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch().into_string().unwrap();
    assert!(response.contains(r#"See <a href="/hello/Foo Bar">Foo Bar</a>."#));
}
