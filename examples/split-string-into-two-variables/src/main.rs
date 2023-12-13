fn main() {
    for address in [
        "localhost:5000",
        "localhost:",
        "localhost:5000:garbage",
        "localhost",
    ] {
        short_with_split_once_and_some(address);
        with_split(address);
        with_split_once_and_some(address);
        with_split_once_and_unwrap(address);
    }
}

// The long winded version.
// It will call the optional else part if the number of `:` is not exactly 1.
fn with_split(text: &str) {
    println!("{text}");
    let parts = text.split(':').collect::<Vec<&str>>();
    if parts.len() == 2 {
        let (hostname, port) = (parts[0], parts[1]);
        println!("hostname: {hostname}");
        println!("port: {port}");
    } else {
        eprintln!("Could not split properly");
    }
    println!();
}

fn short_with_split_once_and_some(text: &str) {
    println!("{text}");
    if let Some((hostname, port)) = text.split_once(':') {
        println!("hostname: {hostname}");
        println!("port: {port}");
    }
    println!();
}

// A shorter version.
// It will call the optional else part if the number of `:` is not exactly 1.
fn with_split_once_and_some(text: &str) {
    println!("{text}");
    if let Some((hostname, port)) = text.split_once(':') {
        if port.contains(':') {
            eprintln!("Too many separators (:)");
        } else {
            println!("hostname: {hostname}");
            println!("port: {port}");
        }
    } else {
        eprintln!("Could not split properly");
    }
    println!();
}

// The overly confident version
// For "localhost:5000:garbage"  this will return "hostname" and "5000:garbage"
// For "localhost"  this will panic!
fn with_split_once_and_unwrap(text: &str) {
    let (hostname, port) = text.split_once(':').unwrap();
    if port.contains(':') {
        eprintln!("Too many separators (:)");
    } else {
        println!("hostname: {hostname}");
        println!("port: {port}");
    }
    println!();
}
