use std::time::Duration;

fn main() {
    let answer = String::from("Hello World!");

    println!("Before:     {answer} {:p} {:?}", &answer, answer.as_ptr());
    let mut handles = vec![];
    for _ in 1..=3 {
        let answer = answer.clone();
        handles.push(std::thread::spawn(move || {
            println!(
                "{:?} {} {:p} {:?}",
                std::thread::current().id(),
                answer,
                &answer,
                answer.as_ptr()
            );
            std::thread::sleep(Duration::from_millis(10));
        }));
    }
    println!("Started:    {answer} {:p} {:?}", &answer, answer.as_ptr());
    for handle in handles {
        handle.join().unwrap();
    }
    println!("After:      {answer} {:p} {:?}", &answer, answer.as_ptr());
}
