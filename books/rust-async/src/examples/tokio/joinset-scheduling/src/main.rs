async fn say(text: &str, sec: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(sec)).await;
    println!("{text}");
}


#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    println!("start");

    let mut tasks = tokio::task::JoinSet::new();
    tasks.spawn(say("Hello", 3));
    tasks.spawn(say("Hi", 1));
    println!("launched both");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("waited");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("waited again");

    tasks.join_all().await;
    println!("done");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}


