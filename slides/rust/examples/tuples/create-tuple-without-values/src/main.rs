fn main() {
    let mut row: (&str, i32, f32);

    // row.0 = "Blue";
    // partially assigned binding `row` isn't fully initialized

    row = ("Purple", 300, 3.45);
    println!("{:?}", row);

    row = ("Green", 99, 4.1);
    println!("{:?}", row);
}
