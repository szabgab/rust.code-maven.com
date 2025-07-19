fn main() {
    let text = "row 1\nrow 2\nrow 3\n";
    println!("{text}");
    println!("-----");

    let lines = text.lines();
    for line in lines {
        println!("line: {line}");
    }
}
