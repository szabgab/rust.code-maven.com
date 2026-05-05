use super::{Args, Command, run};
use sqlx::{Row, sqlite::SqlitePool};
use std::{
    env,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test(flavor = "current_thread")]
async fn run_adds_a_todo() {
    let database_file = unique_database_file("run-adds-a-todo");
    let args = Args {
        cmd: Some(Command::Add {
            description: "buy milk".to_string(),
        }),
    };

    run(args, database_file.to_string_lossy().into_owned())
        .await
        .unwrap();

    let pool = SqlitePool::connect(&format!("sqlite://{}", database_file.display()))
        .await
        .unwrap();
    let row = sqlx::query("SELECT description, done FROM todos WHERE id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.try_get::<String, _>("description").unwrap(), "buy milk");
    assert!(!row.try_get::<bool, _>("done").unwrap());

    std::fs::remove_file(database_file).unwrap();
}

fn unique_database_file(prefix: &str) -> PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    env::temp_dir().join(format!("sqlx-sqlite-todo-{prefix}-{unique_suffix}.db"))
}
