use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;

const DATABASE_URL: &str = "sqlite://family-tree.db";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| DATABASE_URL.to_string());

    if let Err(err) = run(args, &database_url).await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn run(args: Vec<String>, database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    initialize_database(&pool).await?;

    match args.as_slice() {
        [] => list_people(&pool).await?,
        [cmd] if cmd == "list" => list_people(&pool).await?,
        [cmd, name] if cmd == "add" => {
            add_person(&pool, name).await?;
        }
        [cmd, child_name, father_name] if cmd == "father-of" => {
            set_father_of(&pool, child_name, father_name).await?;
        }
        [cmd, child_name, mother_name] if cmd == "mother-of" => {
            set_mother_of(&pool, child_name, mother_name).await?;
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Usage: cargo run [list] | cargo run add NAME | cargo run father-of CHILD FATHER | cargo run mother-of CHILD MOTHER",
            )
            .into());
        }
    }

    Ok(())
}

async fn initialize_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS people (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            father_id INTEGER REFERENCES people(id),
            mother_id INTEGER REFERENCES people(id)
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn add_person(pool: &SqlitePool, name: &str) -> Result<(), sqlx::Error> {
    ensure_person(pool, name).await?;

    Ok(())
}

async fn set_father_of(
    pool: &SqlitePool,
    child_name: &str,
    father_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let child_id = find_person_id(pool, child_name).await?;
    let father_id = find_person_id(pool, father_name).await?;

    let (child_id, father_id) = match (child_id, father_id) {
        (Some(child_id), Some(father_id)) => (child_id, father_id),
        (None, _) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown child: {child_name}"),
            )
            .into());
        }
        (_, None) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown father: {father_name}"),
            )
            .into());
        }
    };

    sqlx::query(
        "UPDATE people
         SET father_id = ?
         WHERE id = ?",
    )
    .bind(father_id)
    .bind(child_id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn set_mother_of(
    pool: &SqlitePool,
    child_name: &str,
    mother_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let child_id = find_person_id(pool, child_name).await?;
    let mother_id = find_person_id(pool, mother_name).await?;

    let (child_id, mother_id) = match (child_id, mother_id) {
        (Some(child_id), Some(mother_id)) => (child_id, mother_id),
        (None, _) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown child: {child_name}"),
            )
            .into());
        }
        (_, None) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown mother: {mother_name}"),
            )
            .into());
        }
    };

    sqlx::query(
        "UPDATE people
         SET mother_id = ?
         WHERE id = ?",
    )
    .bind(mother_id)
    .bind(child_id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn find_person_id(pool: &SqlitePool, name: &str) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query("SELECT id FROM people WHERE name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| r.get("id")))
}

async fn ensure_person(pool: &SqlitePool, name: &str) -> Result<i64, sqlx::Error> {
    sqlx::query(
        "INSERT INTO people (name)
         VALUES (?)
         ON CONFLICT(name) DO NOTHING",
    )
    .bind(name)
    .execute(pool)
    .await?;

    let row = sqlx::query("SELECT id FROM people WHERE name = ?")
        .bind(name)
        .fetch_one(pool)
        .await?;

    let id: i64 = row.get("id");
    Ok(id)
}

async fn list_people(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query(
        "SELECT
            child.name AS name,
            father.name AS father,
            mother.name AS mother
         FROM people AS child
         LEFT JOIN people AS father ON father.id = child.father_id
         LEFT JOIN people AS mother ON mother.id = child.mother_id
         ORDER BY child.name",
    )
    .fetch_all(pool)
    .await?;

    println!("name | father | mother");
    println!("---- | ------ | ------");

    for row in rows {
        let name: String = row.get("name");
        let father: Option<String> = row.get("father");
        let mother: Option<String> = row.get("mother");

        println!(
            "{} | {} | {}",
            name,
            father.unwrap_or_else(|| "-".to_string()),
            mother.unwrap_or_else(|| "-".to_string()),
        );
    }

    Ok(())
}
