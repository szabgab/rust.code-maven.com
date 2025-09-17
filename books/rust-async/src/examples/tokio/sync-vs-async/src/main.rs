fn sync_func() -> u32 {
    println!("sync_func called");
    42
}

async fn async_func() -> u32 {
    println!("async_func called");
    42
}

fn main() {
    println!("Start");
    let rt = tokio::runtime::Runtime::new().unwrap();

    let value = sync_func();
    println!("Value: {}", value);

    let future = async_func();

    println!("Before block");
    let res =rt.block_on(future);
    println!("Result: {}", res);

    println!("End");
}