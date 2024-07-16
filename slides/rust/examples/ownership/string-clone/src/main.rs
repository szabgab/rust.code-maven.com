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
    let x = String::from("Foo Bar");
    prt!(x);
    let y = x.clone();
    prt!(x);
    prt!(y);
}
