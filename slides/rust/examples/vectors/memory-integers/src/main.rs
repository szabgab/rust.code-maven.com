macro_rules! prt {
    ($var: expr) => {
        println!(
            "{:2} {:2} {:p} {:?} {:?}",
            $var.len(),
            $var.capacity(),
            &$var,
            $var.as_ptr(),
            $var
        );
    };
}

macro_rules! prtn {
    ($var: expr) => {
        println!(
            "{:2} {:2} {:p} {:?} {:?}",
            $var.len(),
            $var.capacity(),
            &$var,
            $var.as_ptr(),
            $var
        );
        for ix in 0..$var.len() {
            println!("{:2}    {:p} {:?}", ix, &$var[ix], $var[ix]);
        }
    };
}
fn main() {
    let mut numbers: Vec<i32> = vec![19, 23];

    prtn!(numbers);
    println!();
    let x = String::from("hi");
    prt!(x);

    numbers.extend([1, 2, 3, 4]);
    prtn!(numbers);
    println!();

    numbers.extend([5]);
    prtn!(numbers);
    println!();
}
