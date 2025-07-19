fn main() {
    let x = "abc";
    let y = "abd";
    let z = "abd";
    println!("{}", x < y);
    println!("{:?}", x.cmp(y));
    println!("{}", y == z);
    println!("{:?}", y.cmp(y));
    println!();

    let x = String::from("abc");
    let y = String::from("abd");
    let z = String::from("abd");
    println!("{}", x < y);
    println!("{:?}", x.cmp(&y));
    println!("{}", y == z);
    println!("{:?}", y.cmp(&y));
}
