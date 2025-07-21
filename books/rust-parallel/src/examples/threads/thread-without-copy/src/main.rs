use std::sync::Arc;
use std::time::Duration;

fn main() {
    let mut text = String::from("x");

    // This will create a string that fills the memory.
    // One more iteration and it would crash the program
    for i in 0..=33 {
        text.push_str(&text.clone());
        println!("len: {} {}", i, text.len());
    }

    // Without Arc the thread creates a copy of text pushing it over the
    // total size of memory and crashing the program.
    // With Arc the long string is not copied and thus the process can run.
    let text = Arc::new(text);

    println!("Before:     {} {:p} {:?}", text.len(), &text, text.as_ptr());
    let mut handles = vec![];
    for _ in 1..=3 {
        let text: Arc<String> = text.clone();
        handles.push(std::thread::spawn(move || {
            println!(
                "{:?} {} {:p} {:?}",
                std::thread::current().id(),
                text.len(),
                &text,
                text.as_ptr()
            );
            std::thread::sleep(Duration::from_millis(10));
        }));
    }
    println!("Started:    {} {:p} {:?}", text.len(), &text, text.as_ptr());
    for handle in handles {
        handle.join().unwrap();
    }
    println!("After:      {} {:p} {:?}", text.len(), &text, text.as_ptr());
}
