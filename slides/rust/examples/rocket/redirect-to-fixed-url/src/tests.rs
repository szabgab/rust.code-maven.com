use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn hello_world() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/plain; charset=utf-8"
    );
    assert_eq!(response.into_string(), Some("Hello, world!".into()));

    let response = client.get("/other").dispatch();

    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location").unwrap(), "/page");
    assert_eq!(response.into_string(), None);
    //dbg!(response.headers());
}
