
use reqwest::header::USER_AGENT;

fn main() {
    let client = reqwest::blocking::Client::new();

    let res = client
    .get("http://httpbin.org/headers")
    .send().unwrap();
    println!("{}", res.text().unwrap());

    let res = client
    .get("http://httpbin.org/headers")
    .header(USER_AGENT, "Rust Maven 1.42")
    .send().unwrap();
    println!("{}", res.text().unwrap());
}

