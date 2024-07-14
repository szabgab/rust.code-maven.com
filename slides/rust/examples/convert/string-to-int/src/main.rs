fn main() {
    let text = "23";
    println!("{text:?}");
    println!();

    let number: i32 = text
        .parse()
        .unwrap();

    println!("{number}", );
    println!("{}", number + 1);
    println!();

    let number = text
        .parse::<i32>()
        .expect("Could not convert to i32");
    println!("{number}", );
    println!("{}", number + 1);



    let text = "3.14";
    let number = text
        .parse::<i32>()
        .unwrap();
        //.expect("Could not convert to i32");
    println!("{number}", );
}
