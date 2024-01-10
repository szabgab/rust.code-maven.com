use serde::Deserialize;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
struct Entry {
    name: String,
    phone: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("test_ns").use_db("test_db").await?;

    add_to(&db).await?;
    list_all(&db).await?;

    Ok(())
}

async fn add_to(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let data = ("Foo", "12345");

    let (name, phone) = data;

    let response = db
        .query("CREATE entry SET  name=$name, phone=$phone")
        .bind(("name", name))
        .bind(("phone", phone))
        .await?;

    match response.check() {
        Ok(_) => return Ok(()),
        Err(err) => {
            eprintln!("Could not add entry {}", err);
            std::process::exit(2);
        }
    };
}

async fn list_all(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let mut entries = db
        .query("SELECT name, phone FROM type::table($table) ORDER BY name ASC")
        .bind(("table", "entry"))
        .await?;
    let entries: Vec<Entry> = entries.take(0)?;
    for entry in entries {
        println!("{}: {}", entry.name, entry.phone);
    }

    Ok(())
}
