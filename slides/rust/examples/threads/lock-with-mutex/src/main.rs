use std::sync;
use std::thread;

fn main() {
    let text = sync::Mutex::new(String::new());

    let threads = 3;

    thread::scope(|scope| {
        for _ in 1..=threads {
            scope.spawn(|| {
                //println!("{:?}", thread::current().id());
                let mut guarded_text = text.lock().unwrap();
                let extra = format!("{:?} ", thread::current().id());
                guarded_text.push_str(&extra);
            });
        }
    });

    if let Ok(val) = text.into_inner() {
        println!("Text: {val}");
    }
}
