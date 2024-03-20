macro_rules! prt {
    ($var:expr) => {
        println!("{:2} {:2} {:p} {:15?} '{}'", $var.len(), $var.capacity(), &$var, $var.as_ptr(), $var);
    };
}
fn main() {
    let mut text = String::new();
    prt!(text);
    text.push('a');
    prt!(text);

    let name = String::from("foobar");
    prt!(name);

    text.push('b');
    prt!(text);
    text.push_str("123456");
    prt!(text);

    text.push('x');
    prt!(text);

    text.push_str("123456789123143274368741");
    prt!(text);
}

