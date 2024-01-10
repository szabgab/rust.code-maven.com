use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("test_ns").use_db("test_db").await?;

    add_to(db).await?;

    Ok(())
}

async fn add_to(db: Surreal<Db>) -> surrealdb::Result<()> {
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
