fn main() {
    let strings = vec!["foo", "", "bar"];
    for string in strings {

        // negate a boolean value
        if ! string.is_empty() {
            println!("string: {}", string);
        }
    }


    // Toggle
    let mut on = true;
    println!("on: {}", on);
    on = ! on;
    println!("on: {}", on);
    on = ! on;
    println!("on: {}", on);
}
