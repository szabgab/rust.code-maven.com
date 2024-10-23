use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::opt::Resource;
use surrealdb::Surreal;

use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Fruit {
    name: String,
    date: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;
    dbh.use_ns("demo").use_db("demo-time").await?;

    let _response = dbh.query("DELETE fruits").await?.check();
    list(&dbh).await?;

    for name in ["apple", "banana"] {
        let fruit = Fruit {
            name: name.to_owned(),
            date: Utc::now(),
        };
        let _result = dbh.create(Resource::from("fruits")).content(fruit).await?;
        //println!("{}", result);
    }
    list(&dbh).await?;

    let _response = dbh
        .query("DELETE fruits WHERE name=$name")
        .bind(("name", "apple"))
        .await?
        .check();
    //println!("{:?}", response);
    list(&dbh).await?;

    Ok(())
}

async fn list(dbh: &Surreal<Db>) -> surrealdb::Result<()> {
    let utc: DateTime<Utc> = Utc::now();
    let fruits: Vec<Fruit> = dbh.select("fruits").await?;
    println!("List:");
    for fruit in fruits {
        println!("   {:?}", fruit);
        let elapsed = utc - fruit.date;
        println!("{} microseconds", elapsed.num_microseconds().unwrap());
    }

    Ok(())
}
