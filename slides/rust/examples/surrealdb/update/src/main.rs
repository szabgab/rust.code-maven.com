use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
//use surrealdb::opt::Resource;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Fruit {
    name: String,
    color: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.use_ns("demo").use_db("demo-time").await?;

    let _response = db.query("DELETE fruits").await?.check();
    list(&db).await?;
    let mut id = 0;

    let fruits = vec![
        Fruit {
            name: String::from("apple"),
            color: String::from("yellow"),
        },
        Fruit {
            name: String::from("banana"),
            color: String::from("yellow"),
        },
    ];

    for raw_fruit in fruits {
        id += 1;
        let fruit: Fruit = db.create(("fruits", id)).content(raw_fruit).await?.unwrap();
        println!("db: {:?}", fruit);
    }
    list(&db).await?;

    let _response = db
        .query("UPDATE fruits SET color=$color WHERE name=$name")
        .bind(("name", "apple"))
        .bind(("color", "red"))
        .await?
        .check();
    //println!("{:?}", response);
    list(&db).await?;

    let fruit = Fruit {
        name: String::from("apple"),
        color: String::from("green"),
    };

    //let _result = db.update(Resource::from("fruits")).content(fruit).await?;

    let _fruit: Option<Fruit> = db.update(("fruits", 1)).content(fruit).await?;

    list(&db).await?;
    Ok(())
}

async fn list(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let fruits: Vec<Fruit> = db.select("fruits").await?;
    println!("List:");
    for fruit in fruits {
        println!("   {:?}", fruit);
    }
    println!("----------");

    Ok(())
}
