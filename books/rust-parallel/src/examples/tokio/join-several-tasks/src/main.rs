use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let a = wait_random_time(String::from("a"));
    let b = wait_random_time(String::from("b"));
    tokio::join!(a, b);

    println!("---");

    let n = 3;
    let mut jobs = tokio::task::JoinSet::new();
    for i in 0..n {
        let name = format!("x-{i}");
        let job = wait_random_time(name);
        let _ = jobs.spawn(job);
    }
    while let Some(res) = jobs.join_next().await {
        match res {
            Ok(_) => {} //println!("Job finished successfully"),
            Err(e) => println!("Job failed: {e}"),
        }
    }
}

async fn wait_random_time(name: String) {
    let random_int = rand::random::<u64>() % 100;
    println!("{name} waiting for {random_int} milliseconds");
    sleep(Duration::from_millis(random_int)).await;
}
