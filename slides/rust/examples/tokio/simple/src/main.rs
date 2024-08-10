
async fn do_something() {
    println!("Start to do something");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("End   to do something");
}


fn main() {
    println!("Start");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = do_something();

    println!("Before block");
    rt.block_on(future);

    println!("End");
}
