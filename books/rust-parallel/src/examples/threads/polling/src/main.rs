use std::time::Duration;

fn main() {
    println!("Before");
    let mut handles = vec![];
    for _ in 1..=3 {
        handles.push(std::thread::spawn(|| {
            for ix in 0..=4 {
                println!("{:?} {} ", std::thread::current().id(), ix);
                std::thread::sleep(Duration::from_millis(10));
            }
        }));
    }
    // Enable this to see that the other thread might or might not run before
    // the main thread gets the chance to print.
    //std::thread::sleep(Duration::from_millis(10));
    println!("Started");

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    loop {
        println!("In Main");
        if handles.iter().all(|handle| handle.is_finished()) {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    println!("After");
}
