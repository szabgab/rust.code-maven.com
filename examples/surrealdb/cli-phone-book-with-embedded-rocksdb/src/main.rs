use serde::{Deserialize, Serialize};
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    name: String,
    phone: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let database_folder = match std::env::var("DATABASE_PATH") {
        Ok(val) => std::path::PathBuf::from(val),
        Err(_) => {
            let current_dir = std::env::current_dir().unwrap();
            current_dir.join("phonebook.db")
        }
    };

    let db = Surreal::new::<RocksDb>(database_folder).await?;

    db.use_ns("test").use_db("test").await?;

    if args.len() == 4 {
        let command = &args[1];
        if command == "add" {
            let name = &args[2];
            let phone = &args[3];

            let _response = db
                .query("CREATE entry SET  name=$name, phone=$phone")
                .bind(("name", name))
                .bind(("phone", phone))
                .await?;

            return Ok(());
        }
    }

    if args.len() == 2 {
        let command = &args[1];
        if command == "list" {
            let mut entries = db
                .query("SELECT name, phone FROM type::table($table) ORDER BY name ASC")
                .bind(("table", "entry"))
                .await?;
            let entries: Vec<Entry> = entries.take(0)?;
            for entry in entries {
                println!("{}: {}", entry.name, entry.phone);
            }

            return Ok(());
        }
    }

    if args.len() == 3 {
        let command = &args[1];
        if command == "show" {
            let name = &args[2];

            let mut entries = db
                .query("SELECT name, phone FROM type::table($table) WHERE name=$name")
                .bind(("table", "entry"))
                .bind(("name", name))
                .await?;
            let entries: Vec<Entry> = entries.take(0)?;
            for entry in entries {
                println!("{}: {}", entry.name, entry.phone);
            }

            return Ok(());
        }

        if command == "delete" {
            let name = &args[2];

            let _response = db
                .query("DELETE FROM type::table($table) WHERE name=$name")
                .bind(("table", "entry"))
                .bind(("name", name))
                .await?;

            return Ok(());
        }
    }

    eprintln!("Usage: {} add name value", args[0]);
    std::process::exit(1);
}
