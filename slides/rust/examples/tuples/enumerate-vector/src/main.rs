fn main() {
    #[allow(clippy::useless_vec)]
    let fruits = vec!["apple", "banana", "peach"];
    for (index, name) in fruits.iter().enumerate() {
        println!("{index}) {name}");
    }
}
