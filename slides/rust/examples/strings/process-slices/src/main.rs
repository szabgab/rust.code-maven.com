use std::thread;
use std::sync::Arc;

fn main() {
    let text = Arc::new(String::from("The black cat climbed the green tree"));
    let parts = 3;

    println!("len: {} parts: {}", text.len(), parts);

    println!("Before:     {text}");

    let mut handles  = vec![];
    
    let size = text.len() / parts;
    for part in 0..parts {
        let start = part * size;
        let end = if start + size + size <= text.len() { start + size } else { text.len() };
        println!("{start}-{end}");
        let subtext = text.clone();
        handles.push(thread::spawn(move || {
            println!("{:?} {}", thread::current().id(), &subtext[start..end]);
        }));
    
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("After:      {}", text);

}
