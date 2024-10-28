use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::opt::Resource;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageWithId {
    text: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("demo").use_db("demo-2").await?;

    add_message_using_struct_without_id(&db, "A message").await?;
    add_message_using_struct_without_id_using_resource(&db, "B message").await?;

    let messages = get_all(&db).await?;
    for message in &messages {
        println!("{:?}", message);
    }
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].text, "A message");
    assert_eq!(messages[1].text, "B message");

    Ok(())
}

async fn get_all(dbh: &Surreal<Db>) -> surrealdb::Result<Vec<MessageWithId>> {
    let mut response = dbh.query("SELECT * from messages ORDER BY name").await?;
    let messages: Vec<MessageWithId> = response.take(0)?;

    Ok(messages)
}

async fn add_message_using_struct_without_id(db: &Surreal<Db>, text: &str) -> surrealdb::Result<()> {
    let message = Message {
        text: text.to_owned(),
    };

    let result: Option<MessageWithId> = db.create("messages").content(message).await?;
    println!("{:?}", result);
    assert!(result.is_some());
    assert_eq!(result.unwrap().text, text);

    //    let result = db.create(Resource::from("groups")).content(message).await?;

    Ok(())
}

async fn add_message_using_struct_without_id_using_resource(db: &Surreal<Db>, text: &str) -> surrealdb::Result<()> {
    let message = Message {
        text: text.to_owned(),
    };

    let result: surrealdb::Value = db.create(Resource::from("messages")).content(message).await?;
    println!("{:?}", result);
    //assert_eq!(result.unwrap().text, text);

    Ok(())
}
