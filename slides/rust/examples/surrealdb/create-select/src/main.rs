use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::opt::Resource;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    name: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     name: String,
// }

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("demo").use_db("demo-2").await?;

    list(&db).await?;
    add_group(&db, "Mavens").await?;
    //    add_user(&db, "Mavens", "Jane").await?;

    Ok(())
}

async fn list(db: &Surreal<Db>) -> surrealdb::Result<()> {
    println!("list");
    let groups: Vec<Group> = db.select("groups").await?;
    for group in groups {
        println!("{:?}", group);
    }

    //  let mut response = db.query("SELECT * FROM groups").await?;
    // let entries = response.take(0);
    //         println!("{}", entries);
    //     }

    Ok(())
}

async fn add_group(db: &Surreal<Db>, group_name: &str) -> surrealdb::Result<()> {
    println!("Add group '{group_name}'");
    let group = Group {
        name: group_name.to_owned(),
    };

    let result = db.create(Resource::from("groups")).content(group).await?;
    println!("{}", result);

    Ok(())
}

// async fn add_user(db: &Surreal<Db>, user_name: &str, group_name: &str) -> surrealdb::Result<()> {
//     println!("Add user '{user_name}' to group '{group_name}'");
//     // TODO get the ID of the group

//     Ok(())
// }
