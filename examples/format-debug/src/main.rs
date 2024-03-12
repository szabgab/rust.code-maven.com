#[derive(Debug)]
#[allow(unused)]
struct Person {
    name: String,
    birthyear: u16,
    addresses: Vec<String>,
}

fn main() {
    let text = "Hello, world!";
    let integer = 42;
    let boolean = true;
    let tuple = ("text", 42);
    let vector = vec!["Aunt Em", "Uncle Henry", "Betsy Bobbin"];

    let person = Person {
        name: String::from("Mary Jane"),
        birthyear: 1997,
        addresses: vec![String::from("home"), String::from("work")],
    };

    println!("{} {} {}", text, integer, boolean);
    println!("{:?} {:?} {:?}", text, integer, boolean);
    println!("{:#?} {:#?} {:#?}", text, integer, boolean);
    println!();

    println!("{:?}", tuple);
    println!("{:?}", vector);
    println!("{:?}", person);
    println!();

    println!("{:#?}", tuple);
    println!("{:#?}", vector);
    println!("{:#?}", person);
    println!();

    dbg!(text);
    dbg!(integer);
    dbg!(boolean);
    dbg!(tuple);
    dbg!(vector);
    dbg!(person);
}
