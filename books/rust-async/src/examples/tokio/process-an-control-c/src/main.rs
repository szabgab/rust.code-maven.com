use tokio::select;

#[tokio::main]
async fn main() {
    let mut count = 0;
    loop {
        select! {
            _ = wait_a_bit() => println!("Waited a bit!"),
            _ = tokio::signal::ctrl_c() => {
                    count += 1;
                    if count >= 2 {
                        println!("Exiting...");
                        break;
                    }
                    println!("Received Ctrl+C! Press Ctrl-C again if you'd like to quit?");
            }
        }
    }
}

async fn wait_a_bit() {
    println!("Waiting a bit...");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}





