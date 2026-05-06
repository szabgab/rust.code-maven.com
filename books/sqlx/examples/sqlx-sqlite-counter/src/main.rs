use sqlx::{Row, SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

const DATABASE_URL: &str = "sqlite://counter.db";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(err) = run(args, DATABASE_URL).await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn run(args: Vec<String>, database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS counters (
            name TEXT PRIMARY KEY,
            number INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    match args.as_slice() {
        [] => list_counters(&pool).await?,
        [name] => increment_counter(&pool, name).await?,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Usage: cargo run [NAME]",
            )
            .into());
        }
    }

    Ok(())
}

async fn increment_counter(pool: &SqlitePool, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO counters (name, number)
         VALUES (?, 1)
         ON CONFLICT(name) DO UPDATE SET number = number + 1",
    )
    .bind(name)
    .execute(pool)
    .await?;

    let row = sqlx::query("SELECT number FROM counters WHERE name = ?")
        .bind(name)
        .fetch_one(pool)
        .await?;

    let number: i64 = row.get("number");
    println!("{name} {number}");

    Ok(())
}

async fn list_counters(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query("SELECT name, number FROM counters ORDER BY name")
        .fetch_all(pool)
        .await?;

    for row in rows {
        let name: String = row.get("name");
        let number: i64 = row.get("number");
        println!("{name} {number}");
    }

    Ok(())
}
