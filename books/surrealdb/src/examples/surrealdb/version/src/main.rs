//use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    //let db = Surreal::new::<Ws>("localhost:8000").await?;
    let db = Surreal::new::<Mem>(()).await?;

    let version = db.version().await?;

    println!("{version:?}");
    println!("{version}");

    Ok(())
}
