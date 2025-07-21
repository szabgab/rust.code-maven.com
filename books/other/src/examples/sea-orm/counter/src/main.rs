use sea_orm::{ConnectOptions, Database, DbErr};
// cargo install sea-orm-cli

use futures::executor::block_on;

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}

async fn run() -> Result<(), DbErr> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 2 {
        println!("Usage: {} [NAME]", args[0]);
        std::process::exit(1);
    }

    let opt = ConnectOptions::new("sqlite:./counter.db?mode=rwc");
    let _db = Database::connect(opt).await?;

    if args.len() == 1 {
        list_counters();
    } else {
        let name = &args[1];
        increment(name);
    }
    Ok(())
}

fn list_counters() {
    println!("list");
}

fn increment(name: &str) {
    println!("{name}:");
}
