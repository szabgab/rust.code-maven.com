#[derive(Debug)]
struct Point32 {
    x: u32,
    y: u32,
}

impl std::ops::Add for Point32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let a = 7;
    let b = 8;

    let c = 7_i8;
    let d = 29_i8;

    let p1 = Point32 { x: 2, y: 3 };

    let p2 = Point32 { x: 10, y: 20 };

    println!("{}", add(a, b));
    println!("{}", add(c, d));

    println!("{:?}", add(p1, p2));
}

fn add<T>(x: T, y: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    x + y
}
