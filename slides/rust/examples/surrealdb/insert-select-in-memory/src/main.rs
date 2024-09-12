use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::engine::local::Db;
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
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("demo").use_db("insert-select-in-memory").await?;

    create_a_new_message_with_random_id(&db, "Hello World").await?;
    create_a_new_message_with_random_id(&db, "Another message").await?;

    select_all_the_messages(&db).await?;

    Ok(())
}
async fn select_all_the_messages(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let mut response = db.query("SELECT * from messages").await?;
    let messages: Vec<Message> = response.take(0)?;
    for message in &messages {
        println!("{}", message.text);
    }

    Ok(())
}

async fn create_a_new_message_with_random_id(
    db: &Surreal<Db>,
    message: &str,
) -> surrealdb::Result<()> {
    let created: Vec<Record> = db
        .create("messages")
        .content(Message {
            text: message.to_owned(),
        })
        .await?;
    println!("{created:?}");

    Ok(())
}
