use std::ops::Add;
use std::ops::Mul;

#[derive(Debug)]
#[allow(dead_code)]
struct Vector {
    x: i32,
    y: i32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

fn main() {
    let a = Vector { x: 2, y: 3 };

    let b = Vector { x: 4, y: 5 };

    // We need to derive from Debug to make this printable
    println!("a: {:?}", a);
    println!("b: {b:?}");
    dbg!(&a);

    // We need to implement the [Add trait](https://doc.rust-lang.org/std/ops/trait.Add.html)
    let c = a + b;
    println!("c: {c:?}");

    // // We need to implement the [Mul trait](https://doc.rust-lang.org/std/ops/trait.Mul.html)
    let d = c * 2;
    println!("d: {d:?}");

    let e = 2 * d;
    println!("e: {e:?}");
}
