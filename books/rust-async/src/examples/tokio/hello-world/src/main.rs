#[tokio::main]
async fn main() {
    println!("Hello, world!");

    hello_world().await;
}

async fn hello_world() {
    println!("Hello, async world!");
}
