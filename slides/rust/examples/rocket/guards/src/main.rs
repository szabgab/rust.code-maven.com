#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
struct GuardA;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardA {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        println!("from_request in GuardA '{req}'");
        Outcome::Success(GuardA)
    }
}

#[derive(Debug)]
struct OddGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OddGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        println!("from_request in GuardA '{req}'");
        match req.param::<u32>(0) {
            None => {
                println!("none");
                Outcome::Success(OddGuard)
            }
            Some(val) => {
                println!("{val:?}");
                match val {
                    Ok(num) => {
                        if num % 2 == 1 {
                            Outcome::Success(OddGuard)
                        } else {
                            Outcome::Error((Status::BadRequest, ()))
                        }
                    }
                    Err(err) => {
                        println!("{err}");
                        Outcome::Error((Status::BadRequest, ()))
                    }
                }
            }
        }
    }
}

// #[derive(Debug)]
// struct Alfa {
//     number: i32,
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Alfa {
//     type Error = ();

//     async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
//         Outcome::Success(Alfa { number: 23})
//     }
// }

// #[derive(Debug)]
// struct Beta {
//     number: i32,
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Beta {
//     type Error = ();

//     async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
//         let out = req.guard::<Alfa>().await;
//         match out {
//             Outcome::Success(val) => println!("Success: {val:?}"),
//             Outcome::Error(_) => (),
//             Outcome::Forward(_) => (),
//         };

//         Outcome::Success(Beta { number: 42})
//     }
// }

#[get("/")]
async fn index() -> String {
    String::from("Hello, world!")
}

#[get("/guarded")]
async fn guarded(g: GuardA) -> String {
    rocket::info!("{g:?}");
    String::from("Guarded")
}

#[get("/number?<num>")]
async fn number(num: u32, g: OddGuard) -> String {
    rocket::info!("{num} {g:?}");
    String::from("Guarded")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, guarded, number])
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(super::rocket()).unwrap();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }
}
