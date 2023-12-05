fn main() {
    let (first, second) = if std::env::args().count() == 3 {
        (
            std::env::args().nth(1).unwrap(),
            std::env::args().nth(2).unwrap(),
        )
    } else {
        println!("Usage: {} FIRST SECOND", std::env::args().next().unwrap());
        std::process::exit(1);
    };

    println!("Number of elements on the command line {}", std::env::args().count());

    println!("First: {}", first);
    println!("Second: {}", second);
}
