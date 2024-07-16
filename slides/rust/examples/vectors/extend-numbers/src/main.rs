macro_rules! prt {
    ($var:expr) => {
        println!(
            "{:2} {:2} {:p} {:15?} '{:?}'",
            $var.len(),
            $var.capacity(),
            &$var,
            $var.as_ptr(),
            $var
        );
    };
}



fn main() {
    let mut numbers1 = vec![10, 11];
    prt!(numbers1);

    let mut numbers2 = vec![20, 21, 22];
    prt!(numbers2);

    numbers1.extend(&numbers2);
    prt!(numbers1);
    prt!(numbers2);

    numbers2[1] = 33;
    prt!(numbers1);
    prt!(numbers2);
}
