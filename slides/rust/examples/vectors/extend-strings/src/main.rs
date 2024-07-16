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
    let mut fruits1 = vec![String::from("apple"), String::from("banana")];
    prt!(fruits1);

    let fruits2 = vec![String::from("peach"), String::from("kiwi")];
    prt!(fruits2);

    fruits1.extend(&fruits2);
    prt!(fruits1);
//    prt!(fruits1[2]);
    // prt!(fruits2);

    // fruits1[2] = "egg";
    // prt!(fruits1);
    // prt!(fruits2);
}
