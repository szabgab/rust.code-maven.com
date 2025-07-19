struct Point {
    x: i32,
    y: i32,
}

type Polygon = Vec<Point>;

// cannot define inherent `impl` for a type outside of the crate where the type is defined
// impl Polygon {
//     fn length(&self) {
//     }
// }

fn main() {
    let poly: Polygon = vec![
        Point { x: 3, y: 7 },
        Point { x: 10, y: 6 },
        Point { x: 23, y: 12 },
        Point { x: -2, y: -2 },
    ];

    for point in poly {
        println!("x: {} y: {}", point.x, point.y)
    }
}
