use sqlx::sqlite::SqliteOwnedBuf;
use sqlx::{Connection, SqliteConnection};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let command = get_command();
    if command == "create" {
        create().await?;
    } else if command == "restore" {
        restore().await?;
    }

    Ok(())
}

async fn create() -> anyhow::Result<()> {
        let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

        sqlx::raw_sql(
            "CREATE TABLE notes(id INTEGER PRIMARY KEY, body TEXT NOT NULL);
            INSERT INTO notes(body) VALUES ('hello'), ('world');",
        )
        .execute(&mut conn)
        .await?;

        let snapshot: SqliteOwnedBuf = conn.serialize(None).await?;
        let bytes: &[u8] = snapshot.as_ref();
        std::fs::write("snapshot.db", bytes)?;
        conn.close().await?;

        Ok(())
}   

async fn restore() -> anyhow::Result<()> {
        let bytes = std::fs::read("snapshot.db")?;
        let owned = SqliteOwnedBuf::try_from(bytes.as_slice())?;
        let mut restored = SqliteConnection::connect("sqlite::memory:").await?;
        restored.deserialize(None, owned, false).await?;

        let rows = sqlx::query_as::<_, (i64, String)>("select id, body from notes order by id")
            .fetch_all(&mut restored)
            .await?;

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].1, "hello");
        assert_eq!(rows[1].1, "world");

        Ok(())
}

fn get_command() -> String {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} [create|restore]", args[0]);
        std::process::exit(1);
    }
    let command = &args[1];
    if command != "create" && command != "restore" {
        eprintln!("Invalid command: {}", command);
        eprintln!("Usage: {} [create|restore]", args[0]);
        std::process::exit(1);        
    }

    command.to_string()
}