use surrealdb::Surreal;
use surrealdb::engine::local::Mem;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let _db = Surreal::new::<Mem>(()).await?;

    Ok(())
}
