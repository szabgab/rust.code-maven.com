use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;


#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8001").await?;
    let result = db.query("REMOVE NAMESPACE demo-2;").await?;
    println!("{:?}", result);
    Ok(())
}

    // let args = std::env::args().collect::<Vec<String>>();
    // if args.len() != 2 {
    //     eprintln!("Usage: {} NAMESPACE", args[0]);
    //     std::process::exit(1);
    // }
    // let namespace = args[1].clone();
    // println!("Removing namespace '{namespace}'");

    // let result = db.query("REMOVE NAMESPACE demo-2;");
    // println!("{:?}", result);

    //    let namespace = 
//
//    // Select a specific namespace / database
//    db.use_ns("demo").use_db("demo-2").await?;
//    db.use_ns(ns)
//

//     Ok(())
// }

