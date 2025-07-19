use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn html_page() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    //println!("{html}");
    assert!(html.contains(r#"<link href="/css/style.css" rel="stylesheet">"#));
    assert!(html.contains(r#"<script src="/js/demo.js"></script>"#));
    assert!(html.contains("Hello, <b>world!</b>"));
}

#[test]
fn css_file() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/css/style.css").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/css; charset=utf-8"
    );

    let content = response.into_string().unwrap();
    assert!(content.contains("background-color: #BBBBBB;"));
}

#[test]
fn javascript_file() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/js/demo.js").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/javascript"
    );

    let content = response.into_string().unwrap();
    assert_eq!(content, r#"console.log("hello");"#);
}

#[test]
fn favicon_ico() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/favicon.ico").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "image/x-icon"
    );
}
