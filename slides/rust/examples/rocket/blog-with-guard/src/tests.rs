use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn check_index() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    assert!(html.contains(r#"<a href="/blog/main">main</a><br>"#));
    assert!(html.contains(r#"<a href="/blog/missing">missing</a><br>"#));
}

#[test]
fn check_main() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/blog/main").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    //println!("{html}");
    assert!(html.contains(r#"slug: "main" content: Main page"#));
}

#[test]
fn check_about() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/blog/about").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    //println!("{html}");
    assert!(html.contains(r#"slug: "about" content: About page"#));
}

#[test]
fn check_missing_page() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/blog/other").dispatch();

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    println!("{html}");
    assert!(html.contains(r#"<title>404 Not Found</title>"#));
}
