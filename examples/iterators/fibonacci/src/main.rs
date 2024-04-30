#[derive(Debug)]
struct Fibonacci {
    next_value: u8,
    current: u8,
}

impl Fibonacci {
    fn new() -> Self {
        Self { next_value: 1, current: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_value) = self.current.checked_add(self.next_value) {
            self.current = self.next_value;
            self.next_value = next_value;
            Some(self.current)
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
