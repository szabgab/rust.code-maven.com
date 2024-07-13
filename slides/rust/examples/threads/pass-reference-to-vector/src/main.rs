use std::sync::Arc;

fn main() {
    let animals = Arc::new(vec![
        String::from("mouse"),
        String::from("elephant"),
        String::from("cat"),
        String::from("dog"),
        String::from("giraffe"),
    ]);

    println!("{:?}", animals);
    {
        let animals = animals.clone();
        let handle = std::thread::spawn(move || {
            list_animals(&animals);
        });
        handle.join().unwrap();
    }
    println!("{:?}", animals);
}

fn list_animals(animals: &Vec<String>) {
    for animal in animals {
        println!("{}", animal);
    }
}
