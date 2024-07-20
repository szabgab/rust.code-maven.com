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

    std::thread::scope(|scope| {
        scope.spawn(|| list_animals(&animals));
        scope.spawn(|| list_animals(&animals));
        scope.spawn(|| list_animals(&animals));
    });

    prt!("After:", animals);
}

fn list_animals(animals: &Vec<String>) {
    // Enable this to show that they work in parallel
    // for animal in animals {
    //     println!(" {} in {:?}", animal, std::thread::current().id());
    //     std::thread::sleep(std::time::Duration::from_millis(rand::random::<u8>() as u64));
    // }

    prt!(format!("{:?}", std::thread::current().id()), animals);
}
