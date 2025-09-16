#[tokio::main]
async fn main() {
    let result = multi_step(0).await;   
    println!("Final result: {}", result);
}



async fn multi_step(input: u64) -> u64 {
    let step1 = get_data(input).await;
    let step2 = get_data(step1).await;
    let step3 = get_data(step2).await;
    step3
}

async fn get_data(input: u64) -> u64 {
    // pretend we get some data from the network
    let random_int = rand::random::<u64>() % 11;
    tokio::time::sleep(tokio::time::Duration::from_millis(random_int)).await;
    input + random_int
}