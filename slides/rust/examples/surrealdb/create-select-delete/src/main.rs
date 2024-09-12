use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::Resource;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Fruit {
    name: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.use_ns("demo").use_db("demo-time").await?;

    let _response = db.query("DELETE fruits").await?.check();
    list(&db).await?;

    for name in ["apple", "banana"] {
        let fruit = Fruit {
            name: name.to_owned(),
        };
        let _result = db.create(Resource::from("fruits")).content(fruit).await?;
        //println!("{}", result);
    }
    list(&db).await?;

    let _response = db
        .query("DELETE fruits WHERE name=$name")
        .bind(("name", "apple"))
        .await?
        .check();
    //println!("{:?}", response);
    list(&db).await?;

    Ok(())
}

async fn list(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let fruits: Vec<Fruit> = db.select("fruits").await?;
    println!("List:");
    for fruit in fruits {
        println!("   {:?}", fruit);
    }

    Ok(())
}
