fn main() {
    let a = 8;
    let b = 2;
    let c = 0;
    let x = a / b;
    println!("{x}");

    #[allow(unconditional_panic)]
    let y = a / c;
    println!("{y}");
}
