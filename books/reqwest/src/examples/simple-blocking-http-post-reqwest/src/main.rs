use reqwest::blocking::Client;
use std::collections::HashMap;

fn main() {
    let client = Client::new();
    let params = HashMap::from([
        ("text", "Hello World!"),
    ]);

    match client.post("http://httpbin.org/post").form(&params).send() {
        Ok(resp) => {
            let data:  serde_json::Value = resp.json().unwrap();
            println!("{:#?}", data);
        },
        Err(err) => eprintln!("{}", err),
    }
}
