use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn hello_world() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    assert!(html.contains("<title>Rocket with Tera</title>"));
    assert!(html.contains("<h1>Rocket with Tera</h1>"));
}

#[test]
fn page_not_found() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/other").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    //assert_eq!(html, "");
    assert!(html.contains("<title>404 Not Found</title>"));
    assert!(html.contains("<h1>404 Not Found</h1>"));
    assert!(html.contains("<div>Page not found</div>"));
}

