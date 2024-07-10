fn main() {
    let texts = [String::from("Hello"), String::from("Abc"), String::from("שלום")];
    for text in texts {
        let reversed = reverse(&text);
        let original = reverse(&reversed);
        println!("{text:6} - {reversed:6} - {original:6}");
    }
}

fn reverse(text: &str) -> String {
    let reversed: String = text.chars().rev().collect();
    reversed
}
