
fn main() {
    let text = String::from("One=Two=Three");

    let parts: Vec<&str> = text.split('=').collect();
    println!("{}", parts[0]);
    println!("{}", parts[1]);
    println!("{}", parts[2]);
    println!("-------");

    let text = String::from("mouse cat   oliphant");
    let parts = text.split_whitespace().collect::<Vec<_>>();
    println!("{:?}", parts);
    println!("{}", parts[0]);

}

