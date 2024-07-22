fn main() {
    let threads = 10;
    let limit = 1000;

    let result = count_with_messages(threads, limit);
    println!("{}", result);
    assert_eq!(result, threads*limit);
}

fn count_with_messages(threads: i32, limit: i32) -> i32 {
    let mut counter = 0;
    let (tx, rx) = std::sync::mpsc::channel();
    for _ in 1..=threads {
        let local_tx = tx.clone();

        std::thread::spawn(move || {
            //println!("{:?}", std::thread::current().id());
            for _ in 1..=limit {
                local_tx.send(1).unwrap();
            }
        });
    }

    drop(tx); // we need to close this Sender so the next loop will end properly

    for received in rx {
        counter += received;
    }

    counter
}
