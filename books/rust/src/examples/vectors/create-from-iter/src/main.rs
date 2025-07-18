fn main() {
    let animals = Vec::from_iter(
        ["mouse", "elephant", "cat", "dog", "giraffe"].map(|animal| animal.to_owned()),
    );

    println!("animals: {:?}", animals);
}
