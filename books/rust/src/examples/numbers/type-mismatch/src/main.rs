fn main() {
    let x: i32 = 3;
    let y: i64 = 7;

    let z = x + 1;
    assert_eq!(z, 4);
    println!("{z}");

    let z = y + 1;
    assert_eq!(z, 8);
    println!("{z}");

    //let z = x + y;
    //println!("{z}");
}
