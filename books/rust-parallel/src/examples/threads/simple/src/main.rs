use std::thread;

fn main() {
    println!("Before starting: {:?}", thread::current().id());

    thread::spawn(|| {
        println!("In thread {:?}", thread::current().id());
    });

    println!("After ending: {:?}", thread::current().id());
}
