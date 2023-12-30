use serde::{Deserialize, Serialize};
use surrealdb::engine::local::RocksDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    name: String,
    phone: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
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

            let _created: Vec<Record> = db
                .create("entry")
                .content(Entry {
                    name: name.to_owned(),
                    phone: phone.to_owned(),
                })
                .await?;
            //dbg!(created);

            return Ok(());
        }
    }

    if args.len() == 2 {
        let command = &args[1];
        if command == "list" {
            //let entries: Vec<Record> = db.select("entry").await?;
            //dbg!(entries);

            let mut entries = db
                .query("SELECT name, phone FROM type::table($table)")
                .bind(("table", "entry"))
                .await?;
            //dbg!(&entries);
            //println!("{}", entries.num_statements());
            let entries: Vec<Entry> = entries.take(0)?;
            //println!("{}", entries.len());
            //dbg!(&entries);
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
            //dbg!(&entries);
            //println!("{}", entries.num_statements());
            let entries: Vec<Entry> = entries.take(0)?;
            //println!("{}", entries.len());
            //dbg!(&entries);
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
            //dbg!(response);

            return Ok(());
        }
    }

    eprintln!("Usage: {} add name value", args[0]);
    std::process::exit(1);
    //Ok(())
}
