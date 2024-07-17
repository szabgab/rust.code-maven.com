#[derive(Debug)]
struct Animal {
    name: String,
    size: String,
    weight: i32,
}

fn main() {
    let eli = Animal {name: String::from("elephant"), size: String::from("huge"), weight: 100};
    println!("{}", eli.name);
    println!("{}", eli.size);
    println!("{}", eli.weight);


    println!("{:?}", eli);
    dbg!(eli);
}


