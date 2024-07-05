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
        eprintln!("Usage: {} count []", args[0]);
        process::exit(1);
    }
    let n = args[1].parse::<u64>().expect(format!("Could not convert {} to integer", args[1]).as_str());


    let start = Instant::now();
    if args[2] == "linear" {
        println!("single thread");
        for _x in 1..=10 {
            println!("{n}: {}", fibonacci(n));
        }
    } else if args[2] == "threads" {
        println!("multiple threads");
        let (tx, rx) = mpsc::channel();

        for _x in 1..=10 {
            let txr = tx.clone();
            thread::spawn(move || {
                let res = fibonacci(n);
                txr.send(res.to_string()).unwrap();
                println!("spawned thread finished");
            });
        }
        drop(tx);
    
        for received in rx {
            println!("Got: {}", received);
        }    
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
