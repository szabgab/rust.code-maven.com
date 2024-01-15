struct Point {
    x: f64,
    y: f64,
}

struct Line {
    a: Point,
    b: Point,
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

}
