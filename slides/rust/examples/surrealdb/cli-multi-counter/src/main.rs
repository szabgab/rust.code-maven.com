use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;


#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    name: String,
    count: u32,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let database_folder = match std::env::var("DATABASE_PATH") {
        Ok(val) => std::path::PathBuf::from(val),
        Err(_) => {
            let current_dir = std::env::current_dir().unwrap();
            current_dir.join("db")
        }
    };

    let db = Surreal::new::<RocksDb>(database_folder).await?;

    db.use_ns("counter_ns").use_db("counter_db").await?;

    // Maybe do this only when we create the database
    let _response = db
        .query("DEFINE INDEX counter_name ON TABLE counter COLUMNS name UNIQUE")
        .await?;

    if 2 < args.len() {
        eprintln!("Usage: {} NAME     to count NAME", args[0]);
        eprintln!("       {}          to list all the counters", args[0]);
        std::process::exit(1);
    }

    if 2 == args.len() {
        increment(&db, &args[1]).await?;
        return Ok(());
    }

    println!("Listing counters");
    println!("----------------");
    let mut entries = db
        .query("SELECT name, count FROM counter ORDER BY count DESC")
        .await?;
    let entries: Vec<Entry> = entries.take(0)?;
    for entry in entries {
        println!("{}: {}", entry.name, entry.count);
    }

    Ok(())
}

async fn increment(db: &Surreal<Db>, name: &str) -> surrealdb::Result<()> {
    let response = db
        .query("INSERT INTO counter (name, count) VALUES ($name, $count) ON DUPLICATE KEY UPDATE count += 1;")
        .bind(("name", name.to_owned()))
        .bind(("count", 1))
        .await?;

    match response.check() {
        Ok(mut entries) => {
            let entries: Vec<Entry> = entries.take(0)?;
            // fetching the first (and hopefully only) entry
            if let Some(entry) = entries.into_iter().next() {
                println!("{}", entry.count);
            }

            Ok(())
        }
        Err(err) => {
            eprintln!("Could not add entry {}", err);
            std::process::exit(2);
        }
    }
}
