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
    let html = response.into_string().unwrap();
    assert!(html.contains(r#"<a href="/user/42">User 42</a><br>"#));
}

#[test]
fn valid_user() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/user/42").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    assert_eq!(html, "User ID: 42");
}

#[test]
fn invalid_user() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/user/foo").dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity); // 422
}
