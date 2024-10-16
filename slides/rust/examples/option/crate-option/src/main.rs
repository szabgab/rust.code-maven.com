fn main() {
    //let x = None;

    for value in [None, Some(42)] {
        println!("{value:?}");

        match value {
            None => println!("It was None"),
            Some(number) => println!("It was this: {number} number"),
        };
    }

    println!();

    for value in [None, Some(3.1)] {
        println!("{value:?}");

        match value {
            None => println!("It was None"),
            Some(number) => println!("It was this: {number} number"),
        };
    }
}
