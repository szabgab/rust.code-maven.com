use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn counter() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    assert_eq!(response.into_string().unwrap(), "Rocket Counter: 0");


    let response = client.get("/").dispatch();
    assert_eq!(response.into_string().unwrap(), "Rocket Counter: 1");

    let response = client.get("/").dispatch();
    assert_eq!(response.into_string().unwrap(), "Rocket Counter: 2");
}

#[test]
fn _another_counter() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    assert_eq!(response.into_string().unwrap(), "Rocket Counter: 0");


    let response = client.get("/").dispatch();
    assert_eq!(response.into_string().unwrap(), "Rocket Counter: 1");
}
