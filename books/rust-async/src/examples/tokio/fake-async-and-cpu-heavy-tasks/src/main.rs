
#[tokio::main]
async fn main() {
      let start = std::time::Instant::now();

    println!("Start");
    let mut tasks = tokio::task::JoinSet::new();
    for n in 1..=40 {
        tasks.spawn(fake_async_work(format!("task-{n}")));
        
    }   
    for n in 1..=40 {
        tasks.spawn(fake_cpu_heavy_work(format!("task-{n}")));
    }

    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(_) => {} //println!("Task finished successfully"),
            Err(e) => println!("Task failed: {e}"),
        }
    }

    println!("End");
    let elapsed = start.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);

}



async fn fake_async_work(name: String) {
    println!("Simulate some asynchronous work {name} {}", std::thread::current().name().unwrap_or("unknown"));
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Done with async work {name}");
}

async fn fake_cpu_heavy_work(name: String) {
    println!("Simulate some CPU-heavy work {name}");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Done with CPU-heavy work {name}");
}
