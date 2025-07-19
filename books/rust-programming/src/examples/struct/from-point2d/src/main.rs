#![allow(dead_code)]

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&Point2D> for Point3D {
    fn from(item: &Point2D) -> Point3D {
        Point3D {
            x: item.x,
            y: item.y,
            z: 0.0,
        }
    }
}

fn main() {
    let p2 = Point2D { x: -1.1, y: 2.1 };
    println!("{p2:?}");

    let p3 = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    println!("{p3:?}");

    // let d3 = Point3D::from(p3); // This works
    // let d3 = Point3D::from(p2); // Error: mismatched types  expected `Point3D`, found `Point2D`
    // TODO should no it also mention the possibility to create a From or Into trait?

    let d3 = Point3D::from(&p2);
    println!("{d3:?}");

    let d3: Point3D = (&p2).into();
    println!("{d3:?}");

    //let x3 = (&p2).into::<Point3D>();
    let x3 = Into::<Point3D>::into(&p2);
    println!("{x3:?}");
}
