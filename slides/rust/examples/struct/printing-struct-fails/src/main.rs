struct Animal<'a> {
    name: &'a str,
    size: &'a str,
    weight: i32,
}

fn main() {
    let eli = Animal {name: "elephant", size: "huge", weight: 100};
    println!("{}", eli.name);
    println!("{}", eli.size);
    println!("{}", eli.weight);

    // println!("{}", eli);   // `Animal<'_>` doesn't implement `std::fmt::Display`
    // println!("{:?}", eli); // `Animal<'_>` doesn't implement `Debug`
    // dbg!(eli);             // `Animal<'_>` doesn't implement `Debug`
}

