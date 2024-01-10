use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let _db = Surreal::new::<RocksDb>("tempdb").await?;

    // let current_dir = std::env::current_dir().unwrap();
    // let _db = Surreal::new::<RocksDb>(current_dir.join("tempdb")).await?;

    Ok(())
}
