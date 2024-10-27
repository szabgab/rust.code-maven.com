use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::engine::local::Mem;
use surrealdb::RecordId;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageWithId {
    id: RecordId,
    text: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: RecordId,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;

    dbh.use_ns("demo").use_db("insert-select-in-memory").await?;

    create_a_new_message_with_random_id(&dbh, "Hello World").await?;
    create_a_new_message_with_random_id(&dbh, "Another message").await?;

    let messages = select_all_the_messages(&dbh).await?;
    for message in &messages {
        println!("{:?}", message);
    }
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].text, "Another message");
    assert_eq!(messages[1].text, "Hello World");
    println!();

    let messages = select_all_the_messages_with_id(&dbh).await?;
    for message in &messages {
        println!("{:?}", message);
    }

    Ok(())
}

async fn select_all_the_messages(dbh: &Surreal<Db>) -> surrealdb::Result<Vec<Message>> {
    let mut response = dbh.query("SELECT * from messages ORDER BY text").await?;
    let messages: Vec<Message> = response.take(0)?;

    Ok(messages)
}

async fn select_all_the_messages_with_id(
    dbh: &Surreal<Db>,
) -> surrealdb::Result<Vec<MessageWithId>> {
    let mut response = dbh.query("SELECT * from messages ORDER BY name").await?;
    let messages: Vec<MessageWithId> = response.take(0)?;

    Ok(messages)
}

async fn create_a_new_message_with_random_id(
    dbh: &Surreal<Db>,
    message: &str,
) -> surrealdb::Result<()> {
    let created: Option<Record> = dbh
        .create("messages")
        .content(Message {
            text: message.to_owned(),
        })
        .await?;
    println!("{created:?}");

    Ok(())
}
