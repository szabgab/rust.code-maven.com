use std::sync::Arc;

macro_rules! prt {
    ($text: expr, $var: expr) => {
        println!("{:11} {:?} {:p} {:?}", $text, $var, &$var, $var.as_ptr());
    };
}

fn main() {
    let animals = Arc::new(vec![
        String::from("crab"),
        String::from("ant"),
        String::from("cat"),
        String::from("dog"),
        String::from("bat"),
    ]);

    prt!("Before:", animals);
    let mut handles = vec![];
    for _ in 1..=3 {
        handles.push(std::thread::spawn({
            let animals = animals.clone();
            move || {
                list_animals(&animals);
            }
        }));
    }
    prt!("Started:", animals);
    for handle in handles {
        handle.join().unwrap();
    }
    prt!("After:", animals);
}

fn list_animals(animals: &Vec<String>) {
    prt!(format!("{:?}", std::thread::current().id()), animals);
    //for animal in animals {
    //    println!("{}", animal);
    //}
}
