macro_rules! prt {
    ($var: expr) => {
        println!("{:2} {:2} {:p} {:?}", $var.len(), $var.capacity(), &$var, $var.as_ptr());
    };
}
fn main() {
    let mut animals = vec![String::from("cat"), String::from("dog")];

    prt!(animals);
    prt!(animals[0]);
    prt!(animals[1]);
    println!();

    animals.extend([String::from("mouse")]);

    prt!(animals);
    prt!(animals[0]);
    prt!(animals[1]);
    prt!(animals[2]);
}


