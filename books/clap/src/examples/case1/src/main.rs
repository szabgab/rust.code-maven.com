use clap::{Arg, Command};

fn main() {
    let command = Command::new("My app")
        .arg(Arg::new("name"));
    let matches = command.get_matches();
    match matches.get_one::<String>("name") {
        Some(name) => println!("name: {:?}", name),
        None => println!("No name provided"),
    }
}
