use sea_orm::{ConnectOptions, Database};
// cargo install sea-orm-cli

use futures::executor::block_on;
use sea_orm::{Database, DbErr};


fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}

async fn run()  -> Result<(), DbErr> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 2 {
        println!("Usage: {} [NAME]", args[0]);
        std::process::exit(1);
    }


    let opt = ConnectOptions::new("sqlite:./counter.db?mode=rwc");
    let db = Database::connect(opt).await?;

    if args.len() == 1 {
        list_counters();

    } else {
        let name = &args[1];
        increment(name);
        
    }
}

fn list_counters() {
    println!("list");
}

fn increment(name: &str) {
    println!("{name}:");
}
