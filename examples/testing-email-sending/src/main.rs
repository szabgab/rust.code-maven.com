use clap::Parser;
use lettre::{FileTransport, Message, SendmailTransport, Transport, message::header::ContentType};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    from: String,

    #[arg(long)]
    to: String,
}

fn main() {
    let args = Cli::parse();

    // We get this from the command line.
    // In a real application it would come from a config file.
    let from = &args.from;

    // This is what the user entered when registering.
    let user_email = args.to.clone();

    register(from, &user_email);
}

fn register(admin_email: &str, user_email: &str) -> String {
    // This code is saved to the database and emailed to the user.
    let code = uuid::Uuid::new_v4().to_string();

    let subject = String::from("Email sent to confirm registration");
    let body = format!("Welcome! Use this code {code} to verify your email address.");

    let email = Message::builder()
        .from(admin_email.parse().unwrap())
        .to(user_email.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    #[cfg(test)]
    {
        let dirname = ".";
        let sender = FileTransport::new(dirname);
        let filename = sender.send(&email).unwrap();
        format!("{filename}.eml")
    }

    #[cfg(not(test))]
    {
        println!("Sending email to {}", user_email);
        let sender = SendmailTransport::new();
        sender.send(&email).unwrap();
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_creates_valid_email() {
        let filename = register("admin@example.com", "user@example.com");
        let content = std::fs::read_to_string(filename).unwrap();
        //assert_eq!(content, "");
        assert!(content.contains("From: admin@example.com"));
        assert!(content.contains("To: user@example.com"));
        assert!(content.contains("Subject: Email sent to confirm registration"));
        assert!(content.contains("Welcome! Use this code"));

        let code_start = content.find("Welcome! Use this code ").unwrap() + "Welcome! Use this code ".len();
        let code_end = content.find(" to verify your").unwrap();
        let _code = &content[code_start..code_end];

        // We can then use the code to verify the user in the test.
    }
}