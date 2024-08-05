use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn main_page() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    assert!(response.into_string().unwrap().contains("<h1>Echo</h1>"));
}

#[test]
fn echo_page() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client
        .get("/echo?text=Foo+Bar")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    assert_eq!(
        response.into_string(),
        Some("You typed in <b>Foo Bar</b>".into())
    );
}
