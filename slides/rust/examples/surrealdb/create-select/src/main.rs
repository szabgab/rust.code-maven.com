use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::Resource;
use surrealdb::opt::auth::Root;
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
    println!("Connect to the server");
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    println!("Signin as a namespace, database, or root user");
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    println!("Select a specific namespace / database");
    db.use_ns("demo").use_db("demo-2").await?;

    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            println!("list");
            let groups: Vec<Group> = db.select("groups").await?;
            for group in groups {
                println!("{:?}", group);
            }

            //  let mut response = db.query("SELECT * FROM groups").await?;
            // let entries = response.take(0);
            //         println!("{}", entries);
            //     }
        }
        2 => {
            let group_name = args[1].clone();
            println!("Add group '{group_name}'");
            let group = Group { name: group_name };

            let result = db.create(Resource::from("groups")).content(group).await?;
            println!("{}", result);
        }
        3 => {
            let (user, group) = (&args[1], &args[2]);
            println!("Add user '{user}' to group '{group}'");
        }
        _ => println!("invalid"),
    };

    Ok(())
}
