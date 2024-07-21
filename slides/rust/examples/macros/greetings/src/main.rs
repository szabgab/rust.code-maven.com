macro_rules! greet {
    () => {
        println!("Hello World");   
    };

    (joe) => {
        println!("Welcome Joe!")
    };

    (mary jean) => {
        println!("Hi Mary!")
    }
}

macro_rules! s {
    ($name: expr) => {
        String::from($name)
    };
}


macro_rules! strings {
    ($( $name: expr ),+) => {
        vec![
            $(
                String::from($name),
            )+
            ]
    };
}

fn main() {
    greet!();
    greet!();
    greet!(joe);
    greet!(mary jean);

    let text = s!("Some text");
    println!("{}", text);

    let animals = strings!["cat", "dog"];
    println!("{:#?}", animals);

    // We cannot handle the empty case as that needs type annotation
    // let empty = strings![];
    // println!("{:#?}", empty);

    let mut fruits = strings!["apple"];
    println!("{:#?}", fruits);
    fruits.push(s!["banana"]);
    println!("{:#?}", fruits);
}


