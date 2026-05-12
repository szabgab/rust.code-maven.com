use sqlx::{
    Executor, Row, SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;

static MIGRATOR: Migrator = sqlx::migrate!();

const DATABASE_URL: &str = "sqlite://bank.db";
const USAGE: &str = "Usage: cargo run -- list | cargo run -- add NAME AMOUNT | cargo run -- transfer AMOUNT FROM_NAME TO_NAME";

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
        [cmd] if cmd == "list" => list_accounts(&pool).await?,
        [cmd, name, amount] if cmd == "add" => {
            add_account(&pool, name, parse_non_negative_amount(amount)?).await?;
        }
        [cmd, amount, from_name, to_name] if cmd == "transfer" => {
            transfer(
                &pool,
                parse_positive_amount(amount)?,
                from_name,
                to_name,
                should_panic(),
            )
            .await?;
        }
        _ => {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, USAGE).into());
        }
    }

    Ok(())
}

async fn initialize_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    pool.execute("PRAGMA foreign_keys = ON").await?;
    MIGRATOR.run(pool).await?;
    Ok(())
}

async fn add_account(pool: &SqlitePool, name: &str, amount: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO accounts (name, amount)
         VALUES (?, ?)",
    )
    .bind(name)
    .bind(amount)
    .execute(pool)
    .await?;

    Ok(())
}

async fn list_accounts(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query(
        "SELECT name, amount
         FROM accounts
         ORDER BY name",
    )
    .fetch_all(pool)
    .await?;

    println!("name | amount");
    println!("---- | ------");

    for row in rows {
        let name: String = row.get("name");
        let amount: i64 = row.get("amount");
        println!("{name} | {amount}");
    }

    Ok(())
}

async fn transfer(
    pool: &SqlitePool,
    amount: i64,
    from_name: &str,
    to_name: &str,
    panic_in_middle: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if from_name == to_name {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Source and destination accounts must be different",
        )
        .into());
    }

    let mut tx = pool.begin().await?;

    let from_balance = find_amount(&mut *tx, from_name).await?.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unknown account: {from_name}"),
        )
    })?;

    if find_amount(&mut *tx, to_name).await?.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unknown account: {to_name}"),
        )
        .into());
    }

    if from_balance < amount {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Insufficient funds in {from_name}: has {from_balance}, needs {amount}"),
        )
        .into());
    }

    sqlx::query(
        "UPDATE accounts
         SET amount = amount - ?
         WHERE name = ?",
    )
    .bind(amount)
    .bind(from_name)
    .execute(&mut *tx)
    .await?;

    if panic_in_middle {
        panic!("simulated crash in the middle of transfer");
    }

    sqlx::query(
        "UPDATE accounts
         SET amount = amount + ?
         WHERE name = ?",
    )
    .bind(amount)
    .bind(to_name)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

async fn find_amount<'a, E>(executor: E, name: &str) -> Result<Option<i64>, sqlx::Error>
where
    E: Executor<'a, Database = sqlx::Sqlite>,
{
    let row = sqlx::query("SELECT amount FROM accounts WHERE name = ?")
        .bind(name)
        .fetch_optional(executor)
        .await?;

    Ok(row.map(|record| record.get("amount")))
}

fn parse_non_negative_amount(value: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let amount = value.parse::<i64>().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Amount must be a non-negative integer: {value}"),
        )
    })?;

    if amount < 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Amount must be a non-negative integer: {value}"),
        )
        .into());
    }

    Ok(amount)
}

fn parse_positive_amount(value: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let amount = value.parse::<i64>().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Amount must be a positive integer: {value}"),
        )
    })?;

    if amount <= 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Amount must be a positive integer: {value}"),
        )
        .into());
    }

    Ok(amount)
}

fn should_panic() -> bool {
    matches!(std::env::var("PANIC").as_deref(), Ok("true"))
}
