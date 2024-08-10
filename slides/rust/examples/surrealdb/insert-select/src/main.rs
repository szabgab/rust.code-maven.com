use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    text: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8001").await?;

    // Select a specific namespace / database
    db.use_ns("demo").use_db("demo-1").await?;

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        // Create a new message with a random id
        let created: Vec<Record> = db
            .create("messages")
            .content(Message {
                text: args[1].to_owned(),
            })
            .await?;
        dbg!(created);
    } else {
        // Select all the messages records
        let mut response = db.query("SELECT * from messages").await?;
        let messages: Vec<Message> = response.take(0)?;
        for message in &messages {
            println!("{}", message.text);
        }
    }

    Ok(())
}
