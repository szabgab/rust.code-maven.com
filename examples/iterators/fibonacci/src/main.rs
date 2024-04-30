#[derive(Debug)]
struct Fibonacci {
    last: u8,
    before: u8,
}

impl Fibonacci {
    fn new() -> Self {
        Self { last: 1, before: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_value) = self.before.checked_add(self.last) {
            self.before = self.last;
            self.last = next_value;
            Some(self.before)
        } else {
            None
        }
    }
}

fn main() {
    for fb in Fibonacci::new() {
        println!("{fb}");
        if fb > 30 {
            break;
        }
    }
    println!("------");
    for fb in Fibonacci::new() {
        println!("{fb}");
    }
}
