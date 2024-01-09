use reverse_trait_for_strings_public::Reverse;

fn main() {
    let text = "Welcome to Rust!";
    let reversed = text.reverse();
    println!("{}", reversed);

    let twice = reversed.reverse();
    println!("{}", twice);

    assert_eq!(twice, text);
}
