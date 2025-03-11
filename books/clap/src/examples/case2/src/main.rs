use clap::{Arg, Command};

fn main() {
    let command = Command::new("My app")
        .arg(Arg::new("fname"))
        .arg(Arg::new("lname"));

    let matches = command.get_matches();

    match matches.get_one::<String>("fname") {
        Some(name) => println!("fname: {:?}", name),
        None => println!("No fname provided"),
    }

    match matches.get_one::<String>("lname") {
        Some(name) => println!("lname: {:?}", name),
        None => println!("No lname provided"),
    }
}
