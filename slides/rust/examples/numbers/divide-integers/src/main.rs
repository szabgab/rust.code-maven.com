fn main() {
    let x = 8;
    let y = 3;
    let div = x / y;
    println!("{x} / {y} = {div}"); // 8 / 3 = 2

    let div = x as f32 / y as f32;
    println!("{x} / {y} = {div}");  // 8 / 3 = 2.6666667

}
