use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use rand::{distributions::Alphanumeric, Rng};


fn main() {
    let len = get_len();


    let start = std::time::Instant::now();

    let text: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    println!("{}", text.len());
    //let end = std::time::Instant::now();
    println!("Elapsed time: {:?}", start.elapsed());


    let start = std::time::Instant::now();
    let counter = count(&text);
    if len < 30 {
        println!("{:#?}", counter);
    }
    println!("Elapsed time: {:?}", start.elapsed());


    // let handle = thread::spawn(|| {
    //         println!("Hi number {} from the spawned thread! {:?}", i, thread::current().id());
    //         thread::sleep(Duration::from_millis(1));
    //     }
    //     println!("Spawned thread ended");
    // });


}   


fn get_len() -> usize {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} LENGTH", args[0]);
        std::process::exit(1);
    }
    args[1].parse().unwrap()
}

fn count(text: &str) -> HashMap<char, i32> {
    let mut counter: HashMap<char, i32> = HashMap::new();
    for chr in text.chars() {
        *counter.entry(chr).or_insert(0) += 1;
    }
    counter
}