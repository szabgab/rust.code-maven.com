struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn mv(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn main() {
    let mut pnt = Point {x: 2, y: 3};
    println!("x: {}", pnt.x);
    println!("y: {}", pnt.y);

    pnt.mv(4, 5);
    println!("x: {}", pnt.x);
    println!("y: {}", pnt.y);
}
