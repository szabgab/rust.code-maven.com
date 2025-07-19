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
    assert_eq!(
        response.headers().get_one("Location").unwrap(),
        "/user/23?msg=hello"
    );
    assert_eq!(response.into_string(), None);

    let response = client.get("/redir-value").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(
        response.headers().get_one("Location").unwrap(),
        "/group/42?msg=message"
    );
    assert_eq!(response.into_string(), None);

    let response = client.get("/redir-none").dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location").unwrap(), "/group/42");
    assert_eq!(response.into_string(), None);
}
