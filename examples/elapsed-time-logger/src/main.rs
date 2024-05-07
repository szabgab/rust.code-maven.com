struct ElapsedTimer {
    name: String,
    start: std::time::Instant,
}

impl Drop for ElapsedTimer {
    fn drop(&mut self) {
        let end = std::time::Instant::now();
        let elapsed = end - self.start;
        println!("ENDED {} {}", self.name, elapsed.as_millis());
    }
}

impl ElapsedTimer {
    pub fn new(name: &str) -> Self {
        let timer = Self {
            name: name.to_owned(),
            start: std::time::Instant::now(),
        };
        println!("START {}", timer.name);
        timer
    }
}
fn main() {
    println!("Start");
    do_something(100);
    println!("End");
}


fn do_something(millis: u64) {
    let _a = ElapsedTimer::new("do_something");
    std::thread::sleep(std::time::Duration::from_millis(millis))
}