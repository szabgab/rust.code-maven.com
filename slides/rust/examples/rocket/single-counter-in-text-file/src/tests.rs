use rocket::http::Status;
use rocket::local::blocking::Client;
use tempdir::TempDir;

#[test]
fn test_counte() {
    let tmp_dir = TempDir::new("counter").unwrap();
    std::env::set_var("COUNTER_PATH", tmp_dir.path().join("counter.txt"));

    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Counter is 1".into()));

    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Counter is 2".into()));
}
