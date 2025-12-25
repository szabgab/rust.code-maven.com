use clap::Parser;
use lettre::{FileTransport, Message, SendmailTransport, Transport, message::header::ContentType};
use std::io::Read;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    from: String,

    #[arg(long)]
    to: String,

    #[arg(long)]
    subject: String,

    #[arg(long)]
    dir: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let mut body = String::new();
    std::io::stdin().read_to_string(&mut body).unwrap();

    let email = Message::builder()
        .from(args.from.parse().unwrap())
        .to(args.to.parse().unwrap())
        .subject(args.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    match args.dir {
        Some(dirname) => {
            let sender = FileTransport::new(dirname);
            let filename = sender.send(&email).unwrap();
            println!("{filename}.eml");
        }
        None => {
            let sender = SendmailTransport::new();
            sender.send(&email).unwrap();
        }
    }
}
