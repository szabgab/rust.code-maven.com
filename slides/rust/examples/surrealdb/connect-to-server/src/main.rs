use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Connect to server.");
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    println!("Authenticating as the root user.");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    println!("Select a specific namespace and database.");
    db.use_ns("namespace").use_db("database").await?;

    Ok(())
}
