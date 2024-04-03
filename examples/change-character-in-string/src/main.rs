macro_rules! prt {
    ($var: expr) => {
        println!(
            "len: {:2} capacity: {:2} {:?} {:?}",
            $var.len(),
            $var.capacity(),
            $var.as_ptr(),
            $var
        );
    };
}

fn main() {
    let mut text = String::from("The black cat climed the green tree");
    prt!(text);
    println!("{}", &text[10..=12]);

    text.replace_range(10..=12, "dog");
    prt!(text);

    text.replace_range(10..=10, "D");
    prt!(text);

    text.replace_range(10..=10, "qqq");
    prt!(text);

    text.replace_range(10..=12, "z");
    prt!(text);
    println!();

    let mut text = String::from("🐈🦮🦍🐍🐪");
    prt!(text);
    println!("{}", &text[12..=15]);

    text.replace_range(12..=15, "🦀");
    prt!(text);

    text.replace_range(4..=7, "dog");
    prt!(text);

    text.replace_range(10..=15, "🦀");
}
