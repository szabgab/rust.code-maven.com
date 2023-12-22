use rocket::http::Status;
use rocket::local::blocking::Client;
#[test]
fn hello_world() {
    //assert_eq!(1, 1);
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}
