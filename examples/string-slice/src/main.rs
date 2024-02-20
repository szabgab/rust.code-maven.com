fn main() {
    as_str();
    println!("---");
    as_string();
}

fn as_str() {
    let mut text = "The black cat climbed the green tree";
    println!("{}", text);
    println!("{}", &text[4..9]);
    println!("{}", &text[4..31]);

    {
        let text = &text[4..31];
        println!("{}", text);
    }

    println!("{}", text);

    {
        text = &text[4..25];
        println!("{}", text);
    }
    println!("{}", text);
}

fn as_string() {
    let mut text = String::from("The black cat climbed the green tree");
    println!("{}", text);
    println!("{}", &text[4..9]);
    println!("{}", &text[4..31]);

    {
        let text = &text[4..31]; // here text is going to be &str till the end of the block
        println!("{}", text);
    }

    println!("{}", text);

    {
        text = text[4..25].to_owned();
        println!("{}", text);
    }
    println!("{}", text);
}
