use std::time::Instant;
use std::sync::mpsc;
use std::thread;
use std::env;
use std::process;

fn main() {
    // 40:  7.1 sec vs 1.3 sec
    // 41: 11.4 sec vs 2.1 sec
    // 42: 18.1 sec vs 3.3 sec
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} n [linear|threads]", args[0]);
        process::exit(1);
    }
    let n = args[1].parse::<u64>().expect(format!("Could not convert {} to integer", args[1]).as_str());

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

fn fibonacci(n :u64) -> u64 {
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
            println!("spawned thread finished");
        });
    }
    drop(tx); // need to drop in the main thread

    for received in rx {
        println!("Got: {}", received);
    }    
}
