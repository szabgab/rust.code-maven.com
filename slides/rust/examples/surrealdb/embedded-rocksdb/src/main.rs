use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let _db = Surreal::new::<RocksDb>("tempdb").await?;

    Ok(())
}
