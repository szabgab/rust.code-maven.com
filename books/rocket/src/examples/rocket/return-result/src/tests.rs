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

    assert_eq!(
        response.into_string(),
        Some(r#"<a href="/language/rust">rust</a> <a href="/language/python">python</a>"#.into())
    );
}

#[test]
fn language_rust() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/language/rust").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );

    assert_eq!(response.into_string(), Some("correct".into()));
}

#[test]
fn language_bad() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/language/python").dispatch();

    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );

    assert_eq!(response.into_string(), Some("This is a 400 error".into()));
}
