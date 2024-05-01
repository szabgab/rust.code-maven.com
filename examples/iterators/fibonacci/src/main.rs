#[derive(Debug)]
struct Fibonacci {
    current: u8,
    previous: u8,
}

impl Fibonacci {
    fn new() -> Self {
        Self {
            current: 0,
            previous: 0,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            self.current = 1;
            Some(self.current)
        } else if let Some(next_value) = self.previous.checked_add(self.current) {
            self.previous = self.current;
            self.current = next_value;
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
