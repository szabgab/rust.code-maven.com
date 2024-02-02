use reqwest::header::USER_AGENT;

fn main() {
    let custom = reqwest::redirect::Policy::custom(|attempt| { attempt.stop() });

    //let client = reqwest::blocking::Client::new();
    let client = reqwest::blocking::Client::builder()
    .redirect(custom)
    .build().unwrap();

    let res = client
    .get("http://httpbin.org/cookies/set?name=Foo")
    .header(USER_AGENT, "Rust Maven 1.42")
    .send().unwrap();
    //println!("{}", res.text().unwrap());
    //let c = res.headers().get("Date").unwrap();
    println!("{}", res.headers().get("Date").unwrap().to_str().unwrap());
    println!("{}", res.headers().get("set-cookie").unwrap().to_str().unwrap());
    for (name, value) in res.headers() {
        println!("{}", name.as_str());
    }
}
