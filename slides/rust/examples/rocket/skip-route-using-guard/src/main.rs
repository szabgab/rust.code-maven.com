#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::content;

#[derive(Debug)]
struct OddNumberU32;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OddNumberU32 {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        println!("from_request in OddNumber '{req}'");
        match req.param::<u32>(1) {
            None => {
                println!("none");
                Outcome::Success(Self)
            }
            Some(val) => {
                println!("val: {val:?}");
                match val {
                    Ok(num) => {
                        if num % 2 == 1 {
                            println!("success");
                            Outcome::Success(Self)
                        } else {
                            println!("BadRequest");
                            Outcome::Forward(Status::NotFound)
                        }
                    }
                    Err(err) => {
                        println!("err: {err}");
                        Outcome::Error((Status::BadRequest, ()))
                    }
                }
            }
        }
    }
}

#[get("/")]
fn index() -> content::RawHtml<String> {
    let html = format!(
        r#"
    <a href="/number/23">23</a>
    <a href="/number/42">42</a>
    "#
    );
    content::RawHtml(html)
}

#[get("/number/<num>", rank = 1)]
fn odd_number(num: u32, _x: OddNumberU32) -> Option<content::RawHtml<String>> {
    let html = format!("Odd number: {num:?}");
    return Some(content::RawHtml(html));
}

#[get("/number/<num>", rank = 2)]
fn any_number(num: u32) -> content::RawHtml<String> {
    println!("any_number");
    let html = format!("Any number: {num:?}");
    return content::RawHtml(html);
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
