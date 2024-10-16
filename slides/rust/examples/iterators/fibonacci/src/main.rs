#[derive(Debug)]
struct Fibonacci {
    index: u32,
    current: u32,
    previous: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            index: 0,
            current: 0,
            previous: 0,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            self.index = 1;
            self.current = 1;
        } else if self.index == 1 {
            self.index = 2;
            self.current = 1;
            self.previous = 1;
        } else {
            (self.current, self.previous) = (self.current + self.previous, self.current);
        }

        Some(self.current)
    }
}

fn main() {
    let fibonacci = Fibonacci::new();
    println!("{:?}", &fibonacci);
    for fibo in fibonacci {
        println!("{fibo}");
        if 100 <= fibo {
            break;
        }
    }
}
