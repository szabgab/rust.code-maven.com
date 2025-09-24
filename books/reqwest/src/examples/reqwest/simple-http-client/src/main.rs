fn main() {
    let host = std::env::args().nth(1).unwrap_or("httpbin.org".into());
    let url = if host == "localhost" {
        String::from("http://localhost/get")
    } else {
        format!("https://{}/get", host)
    };

    let res = match reqwest::blocking::get(url) {
        Ok(res) => res,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    };
    println!("{:?}", res.status());
    println!("{:?}", res);
}
