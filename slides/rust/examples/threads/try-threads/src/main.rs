use std::thread;
use std::time::Duration;

fn main() {
    println!("Before starting: {:?}", thread::current().id());

    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("Hi number {} from the spawned thread! {:?}", i, thread::current().id());
            thread::sleep(Duration::from_millis(1));
        }
        println!("Spawned thread ended");
    });

    //thread::sleep(Duration::from_millis(1));
    //thread::sleep(Duration::from_micros(1));
    for i in 1..=5 {
        println!("Hi number {} from the main    thread! {:?}", i, thread::current().id());
        thread::sleep(Duration::from_millis(1));
    }

    println!("Loop in main thread ended");
    handle.join().unwrap(); // waiting for the other thread to end.
    println!("After ending: {:?}", thread::current().id());

    println!("Exiting");
}
