use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::opt::Resource;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Fruit {
    name: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let dbh = Surreal::new::<Mem>(()).await?;

    dbh.use_ns("demo").use_db("demo-time").await?;

    let list = get_list(&dbh).await?;
    show_list(&list);
    assert!(list.is_empty());

    for name in ["apple", "banana"] {
        let fruit = Fruit {
            name: name.to_owned(),
        };
        let _result = dbh.create(Resource::from("fruits")).content(fruit).await?;
        //println!("{}", result);
    }
    let list = get_list(&dbh).await?;
    show_list(&list);
    assert_eq!(list.len(), 2);

    let _response = dbh
        .query("DELETE fruits WHERE name=$name")
        .bind(("name", "apple"))
        .await?
        .check();
    //println!("{:?}", response);
    let list = get_list(&dbh).await?;
    show_list(&list);
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "banana");

    Ok(())
}

async fn get_list(dbh: &Surreal<Db>) -> surrealdb::Result<Vec<Fruit>> {
    let fruits: Vec<Fruit> = dbh.select("fruits").await?;
    Ok(fruits)
}

fn show_list(list: &Vec<Fruit>) {
    println!("List:");
    for fruit in list {
        println!("   {:?}", fruit);
    }
}
