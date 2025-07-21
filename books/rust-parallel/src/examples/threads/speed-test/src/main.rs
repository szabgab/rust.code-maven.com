use std::env;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} n [linear|threads]", args[0]);
        process::exit(1);
    }
    let n = args[1]
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Could not convert {} to integer", args[1]));

    let repetition = 10;

    let start = Instant::now();
    if args[2] == "linear" {
        linear(n, repetition);
    } else if args[2] == "threads" {
        in_threads(n, repetition);
    } else {
        println!("Invalid parameter {}", args[2])
    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn linear(n: u64, repetition: i32) {
    println!("single thread");
    for _x in 1..=repetition {
        println!("{n}: {}", fibonacci(n));
    }
}

fn in_threads(n: u64, repetition: i32) {
    println!("multiple threads");
    let (tx, rx) = mpsc::channel();

    for _x in 1..=repetition {
        let txr = tx.clone();
        thread::spawn(move || {
            let res = fibonacci(n);
            txr.send(res.to_string()).unwrap();
            println!("spawned {:?} finished", thread::current().id());
        });
    }
    drop(tx); // need to drop in the main thread

    for received in rx {
        println!("Received: {}", received);
    }
}
