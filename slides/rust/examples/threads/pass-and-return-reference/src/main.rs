macro_rules! prt {
    ($text: expr, $var: expr) => {
        println!("{:11} {:?} {:p} {:?}", $text, $var, &$var, $var.as_ptr());
    };
}


fn main() {   
    let animals = vec![
        String::from("crab"),
        String::from("ant"),
        String::from("cat"),
        String::from("dog"),
        String::from("bat"),
    ];

    prt!("Before:", animals);

    let handle = std::thread::spawn(move || {
        list_animals(&animals);
        animals
    });

    // Here we cannot access animals
    //prt!("Started:", animals);
    println!("Started:");

    let animals = handle.join().unwrap();
    prt!("After:", animals);
}

fn list_animals(animals: &Vec<String>) {
    prt!(format!("{:?}", std::thread::current().id()), animals);
}
