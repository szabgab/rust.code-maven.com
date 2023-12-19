fn prompt(text: &str, count: u8) {
    println!("prompt '{}' {} times", text, count);
}

macro_rules! prompt {
    ($text: expr, $count: expr) => {
        prompt($text, $count);
    };

    ($text: expr) => {
        prompt($text, 5);
    };
}

fn main() {
    prompt("What is your secret?", 3);

    prompt!("Still with me?", 4);
    prompt!("What is the default?");
}
