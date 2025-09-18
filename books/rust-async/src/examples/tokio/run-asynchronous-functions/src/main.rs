async fn hi() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Hi!");
}


async fn hello() {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Hello!");
}


#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    // As we don't await the functions, they don't run at all and Rust even warns us about it
    // note: futures do nothing unless you `.await` or poll them
    // hello();
    // hi();



    // Run synchronously, first hello waits for 2 second then hi waits for 1 second
    // Total elapsed time is 3 seconds
    // hello().await;
    // hi().await;


    // Run concurrently, both functions start at the same time.
    // However, our program finished before thet can print anything.
    // tokio::spawn(hello());
    // tokio::spawn(hi());


    // Run concurrently, both functions start at the same time.
    // The main program waits long enough to see their output.
    // Not ideal as we have to guess how long to wait.
    // Total elapsed time is 3 seconds.
    // tokio::spawn(hello());
    // tokio::spawn(hi());
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;


    // Run concurrently, both functions start at the same time.
    // hi finishes after 1 second, hello after 2 seconds
    // Total elapsed time is 2 seconds.
    tokio::join!(
        hello(),
        hi(),
    );

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}

