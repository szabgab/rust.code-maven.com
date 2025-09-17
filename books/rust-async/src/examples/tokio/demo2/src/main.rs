use tokio::fs;
use tokio::time::{sleep, Duration};
async fn has_server_log_changed(mut last_modified: Option<std::time::SystemTime>) -> Option<std::time::SystemTime> {
    match fs::metadata("server.log").await {
        Ok(metadata) => {
            let modified = metadata.modified().ok()?;
            if let Some(last) = last_modified {
                if modified > last {
                    println!("server.log has changed!");
                }
            }
            Some(modified)
        },
        Err(_) => {
            println!("server.log not found");
            None
        }
    }
}

// Example usage: poll for changes every second
// async fn poll_server_log_changes() {
//     let mut last_modified = None;
//     loop {
//         last_modified = has_server_log_changed(last_modified).await;
//         sleep(Duration::from_secs(1)).await;
//     }
// }
use tokio::select;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let mut last_modified = None;
    let mut quit = false;
    loop {
        select! {
            input = read_user_input() => {
                println!("User input received: {}", input);
            },
            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C! Exiting...");
                quit = true;
                break;
            },
            new_modified = has_server_log_changed(last_modified) => {
                if let Some(modified) = new_modified {
                    if let Some(last) = last_modified {
                        if modified > last {
                            println!("changed");
                        }
                    }
                    last_modified = Some(modified);
                }
            }
        }
        println!("After select");
        // Add a short sleep to avoid busy looping
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    println!("After loop");
    std::process::exit(0);

}

async fn read_user_input() -> String{
    let mut reader = BufReader::new(tokio::io::stdin());
    println!("Type something and press Enter (async):");
    let mut input = String::new();
    reader.read_line(&mut input).await.unwrap();
    input
}   




// #[tokio::main]
// async fn main() {
//     tokio::spawn(run_callback_in_3_seconds());
//         //     _ = tokio::signal::ctrl_c() => {                    
//     //             if quit {
//     //                 println!("Exiting...");
//     //                 break;
//     //             }
//     //             quit = true;
//     //             println!("Received Ctrl+C! Press Ctrl-C again if you'd like to quit?");
//     //     },

//     loop {
//         wait_a_bit().await;
//     }

//     // let mut quit = false;
//     // select! {
//     //     _ = do_some_work() => println!("done"),
//     //     _ = tokio::signal::ctrl_c() => {                    
//     //             if quit {
//     //                 println!("Exiting...");
//     //                 break;
//     //             }
//     //             quit = true;
//     //             println!("Received Ctrl+C! Press Ctrl-C again if you'd like to quit?");
//     //     },
//     //     _ = run_callback_in_3_seconds() => {
//     //         quit = false;
//     //     }
//     // }
// }
async fn run_callback_in_3_seconds() {
    println!("Setup a callback in 3 seconds...");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    // Place your callback logic here
    std::process::exit(0);
}
async fn do_some_work() {
    loop {
        println!("Doing some work...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}



async fn wait_a_bit() {
        println!("Waiting a bit...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

