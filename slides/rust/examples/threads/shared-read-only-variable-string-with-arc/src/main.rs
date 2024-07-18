use std::sync::Arc;
use std::time::Duration;

fn main() {
    let answer = Arc::new(String::from("Hello World!"));

    println!("Before:     {answer} {:p} {:?}", &answer, answer.as_ptr());
    for _ in 1..=3 {
        let answer = answer.clone();
        let handle = std::thread::spawn(move || {
            println!(
                "{:?} {} {:p} {:?}",
                std::thread::current().id(),
                answer,
                &answer,
                answer.as_ptr()
            );
            std::thread::sleep(Duration::from_millis(10));
        });
        handle.join().unwrap();
    }
    println!("After:      {answer} {:p} {:?}", &answer, answer.as_ptr());
}
