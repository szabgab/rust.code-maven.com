use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    match client.get("https://httpbin.org/get").header("Cookie", "counter=42").send() {
        Ok(resp) => {
            let data:  serde_json::Value = resp.json().unwrap();
            println!("{:#?}", data);
        },
        Err(err) => eprintln!("{}", err),
    }
}

