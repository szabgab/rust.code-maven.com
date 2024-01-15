struct Point {
    x: i32,
    y: i32,
}

struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn length(&self) -> i32 {
        42
    }
}

fn main() {
    let poly = Polygon  {
        points: vec![
            Point {x: 3, y: 7},
            Point {x: 10, y: 6},
            Point {x: 23, y: 12},
            Point {x: -2, y: -2},
        ],
    };

    for point in &poly.points {
        println!("x: {} y: {}", point.x, point.y)
    }

    println!("Length: {}", poly.length());
}
