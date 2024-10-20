use surrealdb::{Error, Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;


#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Connect to server:");
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    println!("Authenticating as the root user:");
    db.signin(Root {
        username: "local",
        password: "secret",
    }).await?;

    println!("Select a specific namespace and database:");
    db.use_ns("namespace").use_db("database").await?;

    Ok(())
}
