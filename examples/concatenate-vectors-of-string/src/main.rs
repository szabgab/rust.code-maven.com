macro_rules! prt {
    ($var:expr) => {
        println!("{:p}, {:?} {:?}", &$var, &$var.as_ptr(), &$var[0].as_ptr());
    };
}

fn main() {
    concat();
    concat_and_use();
}

fn concat() {
    let birds = vec![String::from("turkey"), String::from("duck")];
    let mammals = vec![String::from("cow"), String::from("horse")];
    let fishes = vec![String::from("salmon"), String::from("cod")];
    dbg!(&birds);
    dbg!(&mammals);
    dbg!(&fishes);

    prt!(birds);
    prt!(mammals);
    prt!(fishes);

    let animals = [birds, mammals, fishes].concat();
    dbg!(&animals);
    prt!(animals);
}

fn concat_and_use() {
    let birds = vec![String::from("turkey"), String::from("duck")];
    let mammals = vec![String::from("cow"), String::from("horse")];
    let fishes = vec![String::from("salmon"), String::from("cod")];

    let animals = [birds.clone(), mammals, fishes].concat();
    dbg!(animals);
    dbg!(&birds);
}

// concatenate vectors - merge vectors, join vectors together
