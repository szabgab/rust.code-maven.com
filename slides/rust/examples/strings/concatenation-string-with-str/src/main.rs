macro_rules! prt {
    ($var:expr) => {
        println!(
            "{:2} {:2} {:p} {:15?} '{}'",
            $var.len(),
            $var.capacity(),
            &$var,
            $var.as_ptr(),
            $var
        );
    };
}

fn main() {
    let str1 = "Hello";
    let string1 = String::from("Apple");
    prt!(string1);

    //let text = str1 + string1; // cannot add `String` to `&str`
    let text = string1 + str1;
    prt!(text);
    println!("{text}");
    //println!("{string1}");
}
