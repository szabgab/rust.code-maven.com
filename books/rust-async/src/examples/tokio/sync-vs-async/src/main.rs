fn sync_func() -> u32 {
    println!("sync_func called");
    42
}

async fn async_func() -> u32 {
    println!("async_func called");
    42
}

#[tokio::main]
async fn main() {
    println!("Start");
    let value = sync_func();
    println!("Value: {}", value);

    println!("Before calling async_func");
    let future = async_func();

    println!("Before await");
    let res = future.await;
    println!("Result: {}", res);

    println!("End");
}


