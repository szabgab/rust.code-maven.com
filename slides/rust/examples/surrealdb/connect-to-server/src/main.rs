use surrealdb::{Error, Surreal};
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;


#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("connect to server");
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    println!("Signin as a namespace, database, or root user");
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    println!("Select a specific namespace / database");
    db.use_ns("namespace").use_db("database").await?;

    Ok(())
}
