fn main() {
    let text = String::from("mouse cat   oliphant");
    println!("{text}");

    let parts = text.split(' ');
    //println!("{:?}", parts);
    for part in parts {
        println!("{}", part);
    }
    println!("-------");

    let parts = text.split_whitespace();
    //println!("{:?}", parts);
    for part in parts {
        println!("{}", part);
    }
    println!("-------");


    //println!("{}", parts[0]); // cannot index into a value of type `SplitWhitespace<'_>`

    let parts = text.split_whitespace();
    let parts: Vec<&str> = parts.collect();
    println!("{:?}", parts);
}
