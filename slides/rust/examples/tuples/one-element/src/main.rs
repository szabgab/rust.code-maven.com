fn main() {
    let thing = ("text",);
    println!("{:?}", thing);
    println!("{}", thing.0);

    let thing = "text";
    println!("{}", thing);
}
