#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    num: u32,
}
impl Number {
    fn new(num: u32) -> Self {
        println!("Creating {num}");
        Self { num }
    }
}

impl Drop for Number {
    fn drop(&mut self) {
        println!("Dropping Number! {}", self.num);
    }
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

impl Iterator for Range {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end < self.start {
            return None;
        }

        let current = self.start;
        self.start += 1;

        Some(Number::new(current))
    }
}

fn main() {
    let mut counter = 0;
    let range = Range::new(3, 5);
    for num in range {
        counter += 1;
        println!("{num:?}");
    }
    println!("for loop: {counter}");

    println!("-----");
    println!(
        "collect.len: {}",
        Range::new(3, 5).collect::<Vec<_>>().len()
    );

    println!("-----");
    println!("count: {}", Range::new(3, 5).count());
}
