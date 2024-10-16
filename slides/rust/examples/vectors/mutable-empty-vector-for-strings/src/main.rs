fn main() {
    let mut names = vec![];
    println!("{:?}", names);

    names.push(String::from("apple"));
    // names.push(23);   // error[E0308]: mismatched types - expected `&str`, found integer
    // names.push(3.14); // error[E0308]: mismatched types - expected `&str`, found floating-point number

    names.push(String::from("banana"));

    println!("{:?}", names);
    for name in names {
        println!("{}", name);
    }
}
