use std::sync::Arc;
use std::thread;

macro_rules! prt {
    ($text: expr, $var: expr) => {
        prt!($text, $var, $var)
    };
    ($text: expr, $var: expr, $show: expr) => {
        println!("{:11} {:p} {:?} {:?}", $text, &$var, $var.as_ptr(), $show);
    };
}

fn main() {
    let text = Arc::new(String::from("The black cat climbed the green tree"));
    let parts = 3;

    println!("len: {} parts: {}", text.len(), parts);

    prt!("Before:", text);

    let mut handles = vec![];

    let size = text.len() / parts;
    for part in 0..parts {
        let start = part * size;
        let end = if start + size + size <= text.len() {
            start + size
        } else {
            text.len()
        };
        //println!("{start:3}-{end:3} {}", &text[start..end]);
        
        handles.push(thread::spawn({
            let text = text.clone();
            move || {
            prt!(format!("{:?}", thread::current().id()), text, &text[start..end]);
        }}));
    }
    prt!("Started:", text);

    for handle in handles {
        handle.join().unwrap();
    }

    prt!("After:", text);
}
