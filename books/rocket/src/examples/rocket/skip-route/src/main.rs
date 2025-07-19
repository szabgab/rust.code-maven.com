#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<String> {
    let html = String::from(
        r#"
    <a href="/number/23">23</a>
    <a href="/number/42">42</a>
    "#,
    );
    content::RawHtml(html)
}

#[get("/number/<num>", rank = 1)]
fn odd_number(num: u32) -> Option<content::RawHtml<String>> {
    if num % 2 == 1 {
        let html = format!("Odd number: {num:?}");
        return Some(content::RawHtml(html));
    }

    None
}

#[get("/number/<num>", rank = 2)]
fn any_number(num: u32) -> content::RawHtml<String> {
    let html = format!("Any number: {num:?}");
    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, odd_number, any_number])
}

#[cfg(test)]
mod test {
    use super::rocket;

    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_main() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"<a href="/number/23">23</a>"#));
        assert!(html.contains(r#"<a href="/number/42">42</a>"#));
    }

    #[test]
    fn test_23() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/number/23").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert_eq!(html, r#"Odd number: 23"#);
    }

    #[test]
    fn test_42() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/number/42").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert_eq!(html, r#"Any number: 42"#);
    }
}
