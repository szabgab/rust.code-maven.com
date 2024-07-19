use std::time::Duration;

fn main() {
    let answer = 42;

    println!("Before:     {answer} {:p}", &answer);
    let mut handles = vec![];
    for _ in 1..=3 {
        handles.push(std::thread::spawn(move || {
            println!("{:?} {} {:p}", std::thread::current().id(), answer, &answer);
            std::thread::sleep(Duration::from_millis(10));
        }));
    }
    println!("Started:    {answer} {:p}", &answer);

    for handle in handles {
        handle.join().unwrap();
    }
    println!("After:      {answer} {:p}", &answer);
}
