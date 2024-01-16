struct Point {
    x: f64,
    y: f64,
}

struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn length(&self) -> f64 {
        let x_size = self.a.x - self.b.x;
        let y_size = self.a.y - self.b.y;
        let res = x_size * x_size + y_size * y_size;
        res.sqrt()
    }
}
fn main() {
    let line = Line {
        a: Point {x: 2.0, y: 3.0},
        b: Point {x: 7.0, y: -7.0},
    };

    println!("a.x: {}", line.a.x);
    println!("a.y: {}", line.a.y);
    println!("b.x: {}", line.b.x);
    println!("b.y: {}", line.b.y);

    println!("length: {}", line.length());
}
