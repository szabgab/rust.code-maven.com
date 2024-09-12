use surrealdb::Surreal;
use surrealdb::engine::local::Mem;


#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;
    let result = db.query("REMOVE NAMESPACE demo-2;").await?;
    println!("{:?}", result);
    Ok(())
}

