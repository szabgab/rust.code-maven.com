struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let pnt = Point { x: 2.0, y: 3.0 };
    println!("x: {}", pnt.x);
    println!("y: {}", pnt.y);

    println!("Distance from origo: {}", pnt.distance());
    println!("Distance from origo: {}", pnt.distance());
}
