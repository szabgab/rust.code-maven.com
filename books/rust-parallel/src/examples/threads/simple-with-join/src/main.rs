use std::thread;

fn main() {
    println!("Before starting: {:?}", thread::current().id());

    let handle = thread::spawn(|| {
        println!("In thread {:?}", thread::current().id());
    });

    println!("Before join: {:?}", thread::current().id());

    handle.join().unwrap();

    println!("After ending: {:?}", thread::current().id());
}
