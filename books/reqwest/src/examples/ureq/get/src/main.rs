fn main() {
    let url = get_url();
    println!("GET request to: {}", url);
    let response = ureq::get(&url).call();
    match response {
        Ok(mut resp) => {
            println!("Status: {}", resp.status());

            println!("------- Headers --------");
            resp.headers_mut().iter().for_each(|(k, v)| {
                println!("{}: {:?}", k, v);
            });

            println!("------- Body --------");
            let body = resp.body_mut();
            if let Ok(content) = body.read_to_string() {
                println!("Body: {content}");
            } else {
                eprintln!("Failed to read response body");
                return;
            };
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn get_url() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    args[1].clone()
}
