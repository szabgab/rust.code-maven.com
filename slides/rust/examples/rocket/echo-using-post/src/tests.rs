use rocket::http::{ContentType, Status};
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
        .post("/echo")
        .header(ContentType::Form)
        .body("text=Foo Bar")
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

#[test]
fn echo_page_missing_text() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client
        .post("/echo")
        .header(ContentType::Form)
        //.body("")
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    assert!(response
        .into_string()
        .unwrap()
        .contains("<h1>422: Unprocessable Entity</h1>"));
}
