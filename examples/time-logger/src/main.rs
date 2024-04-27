#![allow(unused)]

struct Timer {
    name: String,
    start: std::time::Instant,
}

// Can I get the name of the function automatically?
// Can I start this without a visible assignment?
impl Timer {
    pub fn new(name: &str) -> Self {
        println!("Start {name}");
        let start = std::time::Instant::now();
        Self {
            name: name.to_string(),
            start,
        }
    }
}


impl Drop for Timer {
    fn drop(&mut self) {
        let end = std::time::Instant::now();
        println!("drop {} elapsed: {}", self.name, (end-self.start).as_micros());
        let filename = "messages.txt";
        match std::fs::File::options().append(true).create(true).open(filename) {
            Ok(mut file) => {},
            Err(_) => {},
        }
    }
}

fn main() {
    do_something();
}


fn do_something()  {
    let t = Timer::new("do_something");

    for number in 2..100 {
        let mut dividers = vec![];
        for n in 1..=number {
            if number % n  == 0 {
                dividers.push(n);
            } 
        }
    } 

}