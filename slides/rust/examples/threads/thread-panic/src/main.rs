use std::env;
use std::thread;

fn main() {
    println!("Before starting: {:?}", thread::current().id());

    let handle = thread::spawn(|| {
        println!("In thread {:?}", thread::current().id());
        if let Ok(val) = env::var("PANIC") {
            panic!("We have a panic {val}");
        }
        42
    });

    println!("Before join: {:?}", thread::current().id());

    //handle.join().unwrap();
    match handle.join() {
        Ok(val) => println!("The thread returned {val:?}"),
        Err(err) => println!("There was a panic in the thread {err:?}"),
    }

    println!("After ending: {:?}", thread::current().id());
}
