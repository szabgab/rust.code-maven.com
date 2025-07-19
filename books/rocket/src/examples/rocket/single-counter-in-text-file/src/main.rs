#[macro_use]
extern crate rocket;

use std::fs::File;
use std::io::Write;

#[get("/")]
fn index() -> String {
    let file = match std::env::var("COUNTER_PATH") {
        Ok(val) => std::path::PathBuf::from(val),
        Err(_) => {
            let current_dir = std::env::current_dir().unwrap();
            current_dir.join("counter.txt")
        }
    };

    let counter: u32 = if file.exists() {
        std::fs::read_to_string(&file)
            .unwrap()
            .trim_end()
            .parse()
            .unwrap()
    } else {
        0
    };
    let counter = counter + 1;

    let mut fh = File::create(file).unwrap();
    writeln!(&mut fh, "{}", counter).unwrap();

    format!("Counter is {}", counter)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod tests;
