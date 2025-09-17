#[tokio::main]
async fn main() {
    let mut tasks = tokio::task::JoinSet::new();
    for i in 0..10 {
        let time = rand::random::<u64>() % 100;
        println!("Task {i} setup for {time}");
        tasks.spawn(async move {
            println!("Task {i} starts");
            tokio::time::sleep(tokio::time::Duration::from_millis(time)).await;
            println!("Task {i} is done");
        });
    }
    tasks.join_all().await;
}
