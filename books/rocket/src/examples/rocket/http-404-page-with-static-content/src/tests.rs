use rocket::form::validate::Contains;
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
    assert_eq!(response.into_string(), Some("Hello, <b>world!</b>".into()));
}

#[test]
fn page_not_found() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/something").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    assert!(html.contains("<title>404 Page not found</title>"));
    assert!(html.contains("<h1>Ooups</h1>"));
}
