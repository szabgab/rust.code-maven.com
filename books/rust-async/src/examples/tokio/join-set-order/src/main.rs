#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    log::info!("Start");
    let mut tasks = tokio::task::JoinSet::new();

    tasks.spawn(async move {
        log::info!("Task long starts");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        log::info!("Task long is done");
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
    log::info!("Long task started");

    for i in 0..5 {
        let time = rand::random::<u64>() % 100;
        log::info!("Task {i} setup for {time}");
        tasks.spawn(async move {
            log::info!("Task {i} starts");
            tokio::time::sleep(tokio::time::Duration::from_millis(time)).await;
            log::info!("Task {i} is done");
        });
    }

    log::info!("All tasks started");
    std::thread::sleep(std::time::Duration::from_secs(1));
    log::info!("Wait done");

    tasks.join_all().await;
    log::info!("End");
}
