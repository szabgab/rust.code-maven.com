use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello");
        tx.send(val).unwrap();

        for i in 1..=10 {
            thread::sleep(Duration::from_millis(1));
            tx.send(i.to_string()).unwrap();
        }

        //  thread::sleep(Duration::from_millis(1));
        println!("Spawned thread ends");
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    // complex way for reveiving
    for _j in 1..=5 {
        let received = rx.recv().unwrap();
        println!("Received: {}", received);
    }
    println!();

    // simple code for receiving
    for received in rx {
        println!("Received: {}", received);
    }
    println!();

    println!("Main thread ends");
}
