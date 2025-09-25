fn main() {
    let url = get_url("get");

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

fn get_url(path: &str) -> String {
    let host = std::env::args().nth(1).unwrap_or("httpbin.org".into());
    let url = if host == "localhost" {
        format!("http://localhost/{path}")
    } else {
        format!("https://{host}/{path}")
    };

    url
}
