fn main() {
    let mut names = vec![String::from("snake")];
    names.push(String::from("crab"));
    println!("{:?}", names);
    for name in names {
        println!("{}", name);
    }
}
