
#[tokio::main]
async fn main() {
    println!("Start");
    tokio::join!(
        count("a", 10),
        count("b", 10),
    );
    println!("End");
}

async fn count(which: &str, n: u32) {
    for i in 0..n {
        print(which,i).await;
    }
}


async fn print(which: &str, n: u32) {
    let random_int = rand::random::<u64>() % 11;
    tokio::time::sleep(tokio::time::Duration::from_millis(random_int)).await;
    println!("{which} {n} random: {random_int}");
}
