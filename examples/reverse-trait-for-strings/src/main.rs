fn main() {
    let text = "Hello, world!";
    println!("{}", text);
    // println!("{}", text.reverse()); // no method named `reverse` found for reference `&str` in the current scope

    println!("{}", reverse_string(text));
    println!("{}", text.reverse());
}

fn reverse_string(text: &str) -> String {
    text.chars().rev().collect::<String>()
}

trait Reverse {
    fn reverse(&self) -> String {
        "".to_string()
    }
}

impl Reverse for str {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}
