use tokio::io::AsyncReadExt;

async fn do_something() {
    println!("Start to do something");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("End   to do something");
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n-1) + fib(n-2)
}

async fn read_file() {
    println!("Start reading");
    let mut fh = tokio::fs::File::open("src/main.rs").await.unwrap();
    let mut content = vec![];
    fh.read_to_end(&mut content).await.unwrap();
    println!("End   reading {} bytes", content.len());

    let n = 40;
    let res = fib(n);
    println!("fib({n}) = {res}");
}

async fn run() {
    //do_something().await;
    //read_file().await;
    tokio::join!(
        do_something(),
        read_file(),
    );
}

fn main() {
    println!("Start");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run();

    println!("Before block");
    rt.block_on(future);

    println!("End");
}
