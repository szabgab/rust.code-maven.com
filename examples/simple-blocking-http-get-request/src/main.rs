
fn main() {
    let res = match reqwest::blocking::get("https://httpbin.org/ip") {
        Ok(res) => res,
        Err(err) => {
            println!("Error {}", err);
            std::process::exit(1);
        }
    };

    println!("{:?}", res);

    println!("status: {:?}", res.status());

    println!("server: {:?}", &res.headers()["server"]);

    match res.text() {
        Ok(val) => println!("{}", val),
        Err(err) => eprintln!("Error: {}", err),
    };

}
