use std::thread;

fn main() {
    println!("Before starting: {:?}", thread::current().id());

    thread::spawn(|| {
        println!("In thread {:?}", thread::current().id());
        for i in 1..10000000 {
            let _n = i + i;
        }
    });

    for i in 1..10000000 {
        let _n = i + i;
    }

    println!("After ending: {:?}", thread::current().id());
}
