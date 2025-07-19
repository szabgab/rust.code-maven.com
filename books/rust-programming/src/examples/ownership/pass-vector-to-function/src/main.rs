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
    let numbers = vec![5, 2, 3];
    prt!(numbers);
    show_vector_ref(&numbers);
    prt!(numbers);
}

// fn move_vector() {
//     let numbers = vec![5, 2, 3];
//     prt!(numbers);
//     show_vector(numbers);
//     //prt!(numbers);
// }
//
// fn show_vector(things: Vec<i32>) {
//     prt!(things);
// }

fn show_vector_ref(things: &Vec<i32>) {
    prt!(things);
}
