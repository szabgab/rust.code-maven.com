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

    let mut fruits2 = vec![
        String::from("peach"),
        String::from("kiwi"),
        String::from("mango"),
    ];
    prt!(fruits2);

    fruits1.extend(fruits2.clone());
    prt!(fruits1);
    prt!(fruits2);

    prt!(fruits1[3]);
    prt!(fruits2[1]);

    fruits2[1] =
        String::from("some fruit with a very long name that requires more memory than we have");
    prt!(fruits1[3]);
    prt!(fruits2[1]);

    prt!(fruits1);
    prt!(fruits2);
}
