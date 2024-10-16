use thousands::Separable;

struct Point {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(format, "x{} {}x", self.x, self.y)
    }
}

fn main() {
    assert_eq!(12345.separate_with_commas(), "12,345");

    assert_eq!("12345".separate_with_commas(), "12,345");

    let p = Point { x: 1234, y: 4567 };
    println!("{p}");
    println!("{}", p.separate_with_commas());
}
