fn main() {
    let mut names: Vec<String> = vec![];
    add(&mut names, "camel");
    add(&mut names, "snake");
    add(&mut names, "crab");

    println!("{:?}", names);
}

fn add(names: &mut Vec<String>, word: &str) {
    names.push(word.to_owned());
    names.push(format!("{}:{}", word, word.len()));
}
