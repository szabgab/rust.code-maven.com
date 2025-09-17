use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;

#[tokio::main]
async fn main() {
    loop {
        select! {
            input = read_user_input() => {
                println!("User input received: {}", input);
            },
            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C! Exiting...");
                break;
            },
        }
        println!("After select");
    }
    println!("After loop");
    std::process::exit(0);
}

async fn read_user_input() -> String {
    let mut reader = BufReader::new(tokio::io::stdin());
    println!("Type something and press Enter (async):");
    let mut input = String::new();
    reader.read_line(&mut input).await.unwrap();
    input
}
