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

    let _response = db
        .query("DEFINE INDEX entry_email ON TABLE entry COLUMNS name UNIQUE")
        .await?;

    let data = vec![("Joe", "123"), ("Jane", "456"), ("Jil", "789")];

    add_to(&db, data).await?;
    list_all(&db).await?;
    println!("-------------");

    update(&db).await?;
    list_all(&db).await?;
    println!("-------------");

    delete(&db).await?;
    list_all(&db).await?;
    println!("-------------");

    add_to(&db, vec![("Joe", "7777777")]).await?;
    println!("this will not happen as the previous statement panics");
    list_all(&db).await?;
    println!("-------------");

    Ok(())
}

async fn add_to(db: &Surreal<Db>, data: Vec<(&str, &str)>) -> surrealdb::Result<()> {
    for (name, phone) in data {
        let response = db
            .query("CREATE entry SET  name=$name, phone=$phone")
            .bind(("name", name.to_owned()))
            .bind(("phone", phone.to_owned()))
            .await?;

        match response.check() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Could not add entry: '{}'", err);
                return Err(err);
            }
        };
    }
    Ok(())
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

async fn update(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let name = "Jane";
    let phone = "55555555";

    let response = db
        .query("UPDATE entry SET phone=$phone WHERE name=$name")
        .bind(("name", name))
        .bind(("phone", phone))
        .await?;

    match response.check() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Could not add entry {}", err);
            Err(err)
        }
    }
}


async fn delete(db: &Surreal<Db>) -> surrealdb::Result<()> {
    let name = "Jane";

    let response = db
        .query("DELETE entry WHERE name=$name")
        .bind(("name", name))
        .await?;

    match response.check() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Could not delete entry {}", err);
            Err(err)
        }
    }
}

