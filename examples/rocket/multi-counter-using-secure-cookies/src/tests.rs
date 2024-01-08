use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_without_cookie() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    // "counter=C3YSnaksM52BVeYOPjvcOp7pNRAez5ZB8aq+a+A%3D; HttpOnly; SameSite=Strict; Path=/; Expires=Mon, 15 Jan 2024 06:44:15 GMT"
    let cookie = response.headers().get_one("set-cookie").unwrap();
    assert!(cookie.contains("counter="));
    assert!(cookie.contains("; HttpOnly; SameSite=Strict; Path=/; Expires="));
    let (head, tail) = cookie.split_once(';').unwrap();
    assert_eq!(head, cookie);
    let (head, tail) = head.split_once('=').unwrap();

    assert_eq!(response.into_string(), Some("Counter: 1".into()));
    


//    let response = client.get("/").cookie(("counter", "41")).dispatch();

//     assert_eq!(response.status(), Status::Ok);
//     assert_eq!(
//         response.headers().get_one("set-cookie").unwrap(),
//         "counter=42; SameSite=Strict; Path=/"
//     );

//     assert_eq!(response.into_string(), Some("Counter: 42".into()));
}


// #[test]
// fn test_with_bad_cookie() {
//     let client = Client::tracked(super::rocket()).unwrap();
//     let response = client.get("/").cookie(("counter", "bla")).dispatch();

//     assert_eq!(response.status(), Status::Ok);
//     assert_eq!(
//         response.headers().get_one("set-cookie").unwrap(),
//         "counter=1; SameSite=Strict; Path=/"
//     );

//     assert_eq!(response.into_string(), Some("Counter: 1".into()));
// }
