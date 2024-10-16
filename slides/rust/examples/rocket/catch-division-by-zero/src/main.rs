#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::content;
use rocket::Request;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
    <form action="/divide" method="GET">
    Divide <input name="a"> by <input name="b"> <input type="submit" value="Divide">
    </form>
    "#,
    )
}

#[get("/divide?<a>&<b>")]
fn divide(a: i32, b: i32) -> content::RawHtml<String> {
    let res = a / b;
    content::RawHtml(format!(r#"{a} / {b} = {res}"#))
}

#[catch(500)]
fn internal_error(status: Status, req: &Request) -> content::RawHtml<String> {
    let reason = status.reason().unwrap_or_default();
    rocket::error!("Error: {reason} in {}", req.uri());
    content::RawHtml(format!(
        r#"
    Internal error: '{reason}' {}
    "#,
        req.uri()
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, divide])
        .register("/", catchers![internal_error])
}

#[cfg(test)]
mod tests {
    use rocket::form::validate::Contains;
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
        assert!(html.contains(r#"<form action="/divide" method="GET">"#));
    }

    #[test]
    fn divide_good() {
        let client = Client::tracked(super::rocket()).unwrap();
        let response = client.get("/divide?a=10&b=5").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.headers().get_one("Content-Type").unwrap(),
            "text/html; charset=utf-8"
        );
        let html = response.into_string().unwrap();
        assert_eq!(html, "10 / 5 = 2");
    }

    #[test]
    fn divide_by_zero() {
        let client = Client::tracked(super::rocket()).unwrap();
        let response = client.get("/divide?a=10&b=0").dispatch();

        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.headers().get_one("Content-Type").unwrap(),
            "text/html; charset=utf-8"
        );
        let html = response.into_string().unwrap();
        println!("{html}");
        assert!(html.contains("Internal error"));
    }
}
