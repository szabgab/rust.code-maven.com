use rocket::form::validate::Contains;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn main() {
    let client = Client::tracked(super::rocket()).unwrap();

    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    assert!(response
        .into_string()
        .unwrap()
        .contains(r#"<a href="/deep/42">number</a> or <a href="/deep/hello">string</a>"#));

    for path in ["deep", "early-match", "early-map"] {
        let response = client.get(format!("/{path}/42")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.headers().get_one("Content-Type").unwrap(),
            "text/html; charset=utf-8"
        );
        assert_eq!(response.into_string(), Some("Number: 42".to_string()));

        let response = client.get(format!("/{path}/hello")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.headers().get_one("Content-Type").unwrap(),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            response.into_string(),
            Some("Message: not a number invalid digit found in string".to_string())
        );
    }
}
