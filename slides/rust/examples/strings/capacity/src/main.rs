macro_rules! prt {
    ($var:expr) => {
        println!(
            "{:2} {:2} {:?}",
            $var.len(),
            $var.capacity(),
            $var,
        );
    };
}


fn main() {
    let mut text = String::new();
    prt!(text);

    text.push('A');
    prt!(text);

    text.push_str(" black ");
    prt!(text);

    text.push_str("cat");
    prt!(text);

    text.push_str(" climebed");
    prt!(text);
}
