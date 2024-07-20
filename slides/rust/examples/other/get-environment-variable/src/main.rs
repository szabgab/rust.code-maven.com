use std::env;

fn main() {
    for name in ["PATH", "RUST"] {
        println!("Checking {name}");
        match env::var(name) {
            Ok(val) => println!("{name}={val}"),
            Err(err) => println!("Environment variable {name} does not exist.\n{err}"),
        }
    }
}
