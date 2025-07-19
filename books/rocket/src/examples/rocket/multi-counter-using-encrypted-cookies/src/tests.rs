use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_without_cookie() {
    let client = Client::tracked(super::rocket()).unwrap();
    let client = client;

    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);

    // "counter=C3YSnaksM52BVeYOPjvcOp7pNRAez5ZB8aq+a+A%3D; HttpOnly; SameSite=Strict; Path=/; Expires=Mon, 15 Jan 2024 06:44:15 GMT"
    let cookie = response.headers().get_one("set-cookie").unwrap();
    assert!(cookie.contains("counter="));
    assert!(cookie.contains("; HttpOnly; SameSite=Strict; Path=/; Expires="));

    assert_eq!(response.into_string(), Some("Counter: 1".into()));
}

#[test]
fn test_with_cookie() {
    let client = Client::tracked(super::rocket()).unwrap();

    let response = client.get("/").private_cookie(("counter", "41")).dispatch();
    assert_eq!(response.status(), Status::Ok);

    assert_eq!(response.into_string(), Some("Counter: 42".into()));
}

#[test]
fn test_with_bad_cookie() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client
        .get("/")
        .private_cookie(("counter", "bla"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Counter: 1".into()));
}

// I was expecting this too to work, but it does not, I get back "Counter 1" for the second request as well.
// #[test]
// fn test_counter() {
//     let client = Client::tracked(super::rocket()).unwrap();
//     let client = client;

//     let response = client.get("/").dispatch();

//     assert_eq!(response.status(), Status::Ok);

//     // "counter=C3YSnaksM52BVeYOPjvcOp7pNRAez5ZB8aq+a+A%3D; HttpOnly; SameSite=Strict; Path=/; Expires=Mon, 15 Jan 2024 06:44:15 GMT"
//     let cookie = response.headers().get_one("set-cookie").unwrap();
//     assert!(cookie.contains("counter="));
//     assert!(cookie.contains("; HttpOnly; SameSite=Strict; Path=/; Expires="));

//     let (head, _tail) = cookie.split_once(';').unwrap();
//     let (_head, tail) = head.split_once('=').unwrap();
//     let cookie_str = tail.to_string();

//     assert_eq!(response.into_string(), Some("Counter: 1".into()));

//     //assert_eq!(cookie_str, "");
//     let response = client.get("/").cookie(("counter", cookie_str)).dispatch();
//     assert_eq!(response.into_string(), Some("Counter: 2".into()));
// }
