macro_rules! prt {
    ($name: expr) => {
        println!("{:?} {:p}", $name, &$name);
    };
}


#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

fn main() {
    let point1 = Point { x: 1, y: 2, z: 3 };
    prt!(point1);

    let point2 = Point { ..point1 };
    prt!(point2);

    let point3 = Point { x: 4, ..point1 };
    prt!(point3);
}
