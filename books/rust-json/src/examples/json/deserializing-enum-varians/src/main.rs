use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RequestData {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PollNotificationsData {
    poll_interval_ms: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum UserFacingApi {
    Request(RequestData),
    PollNotifications(PollNotificationsData),
}

fn main() {
    let request_api = get_request_api("a");
    println!("request: {request_api:?}");

    let notifications_api = get_request_api("b");
    println!("notifications: {notifications_api:?}");
    //notifications: PollNotifications(PollNotificationsData { poll_interval_ms: 1000 })
}
// basic example for deserializing with enum varians from Yoni Peleg

fn get_request_api(which: &str) -> UserFacingApi {
    if which == "a" {
        let request_json = r#"{"Request": {"name": "John", "email": "john.doe@example.com" }}"#;
        let request_api: UserFacingApi = serde_json::from_str(request_json).unwrap();
        request_api
    } else {
        let notification_json = r#"{"PollNotifications": {"poll_interval_ms": 1000 }}"#;
        let notifications_api: UserFacingApi = serde_json::from_str(notification_json).unwrap();
        notifications_api
    }
}

#[test]
fn it_works() {
    use UserFacingApi;

    let request_api = get_request_api("a");
    assert_eq!(
        request_api,
        UserFacingApi::Request(RequestData {
            name: String::from("John"),
            email: String::from("john.doe@example.com")
        })
    );
}
