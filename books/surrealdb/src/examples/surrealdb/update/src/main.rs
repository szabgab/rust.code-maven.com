use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Fruit {
    name: String,
    color: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;
    dbh.use_ns("demo").use_db("demo-time").await?;

    let _response = dbh.query("DELETE fruits").await?.check();
    list(&dbh).await?;
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
        let fruit: Fruit = dbh
            .create(("fruits", id))
            .content(raw_fruit)
            .await?
            .unwrap();
        println!("fruit: {:?}", fruit);
    }
    list(&dbh).await?;

    let _response = dbh
        .query("UPDATE fruits SET color=$color WHERE name=$name")
        .bind(("name", "apple"))
        .bind(("color", "red"))
        .await?
        .check();
    //println!("{:?}", response);
    list(&dbh).await?;

    let fruit = Fruit {
        name: String::from("apple"),
        color: String::from("green"),
    };

    //let _result = dbh.update(Resource::from("fruits")).content(fruit).await?;

    let _fruit: Option<Fruit> = dbh.update(("fruits", 1)).content(fruit).await?;

    list(&dbh).await?;
    Ok(())
}

async fn list(dbh: &Surreal<Db>) -> surrealdb::Result<()> {
    let fruits: Vec<Fruit> = dbh.select("fruits").await?;
    println!("List:");
    for fruit in fruits {
        println!("   {:?}", fruit);
    }
    println!("----------");

    Ok(())
}
