use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    println!("Start");
    let future = run();

    println!("Before await");
    future.await;

    println!("End");
}

async fn run() {
    // sequentially
    do_something().await;
    read_file().await;

    // concurrently
    // tokio::join!(
    //     do_something(),
    //     read_file(),
    // );
}

async fn do_something() {
    println!("Start to do something");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("End   to do something");
}

async fn read_file() {
    println!("Start reading");
    let mut fh = tokio::fs::File::open("src/main.rs").await.unwrap();
    let mut content = vec![];
    fh.read_to_end(&mut content).await.unwrap();
    println!("End   reading {} bytes", content.len());
}
