use std::time::Duration;

fn main() {
    let answer = 42;

    println!("Before:     {answer} {:p}", &answer);
    for _ in 1..=3 {
        let handle = std::thread::spawn(move || {
            println!("{:?} {} {:p}", std::thread::current().id(), answer, &answer);
            std::thread::sleep(Duration::from_millis(10));
        });
        handle.join().unwrap();
    }
    println!("After:      {answer} {:p}", &answer);
}
