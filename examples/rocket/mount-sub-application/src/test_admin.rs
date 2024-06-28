use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn admin_main() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/admin").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Admin main page".into()));
}
