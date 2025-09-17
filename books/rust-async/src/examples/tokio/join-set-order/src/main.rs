#[tokio::main]
async fn main() {
    println!("Start");
    let mut tasks = tokio::task::JoinSet::new();

    tasks.spawn(async move {
        println!("Task long starts");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("Task long is done");
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Long task started");

    for i in 0..5 {
        let time = rand::random::<u64>() % 100;
        println!("Task {i} setup for {time}");
        tasks.spawn(async move {
            println!("Task {i} starts");
            tokio::time::sleep(tokio::time::Duration::from_millis(time)).await;
            println!("Task {i} is done");
        });
    }

    println!("All tasks started");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Wait done");

    tasks.join_all().await;
    println!("End");
}
