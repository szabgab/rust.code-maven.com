#[tokio::main]
async fn main() {
    println!("Start");
    let value = sync_func();
    println!("Value: {}", value);

    let future = async_func();

    println!("Before await");
    let res = future.await;
    println!("Result: {}", res);

    println!("End");
}

fn sync_func() -> u32 {
    println!("sync_func called");
    42
}

async fn async_func() -> u32 {
    println!("async_func called");
    42
}
