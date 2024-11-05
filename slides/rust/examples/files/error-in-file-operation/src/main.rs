fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: {} expect", args[0]);
        std::process::exit(1);
    }
    let command = &args[1];

    let text = "Hello, world!";
    let filename = "other/hello.txt";
    match command.as_str() {
        "nothing" => {
            let _ = std::fs::write(filename, text);
            panic!("This will panic");
        },
        "unwrap" => std::fs::write(filename, text).unwrap(),
        "expect" => std::fs::write(filename, text).expect(format!("Should write text to file {filename:?}").as_str()),
        "match" => match std::fs::write(filename, text) {
            Ok(_) => println!("Write text to file successfully"),
            Err(err) => {
                println!("Failed to write text to file {filename:?}: '{err}'");
                match err.kind() {
                    std::io::ErrorKind::PermissionDenied => println!("Permission denied"),
                    std::io::ErrorKind::NotFound => println!("File not found"),
                    // There are many other error types, but we are not handling them here
                    _ => println!("Other error"),
                }
            }
        },
        other => println!("Unknown command: {}", other),
    }
}
