use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<RocksDb>("tempdb").await?;

    db.use_ns("namespace").use_db("database").await?;

    Ok(())
}
