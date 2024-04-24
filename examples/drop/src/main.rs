#![allow(unused)]

struct Thing {
    name: String,
}

impl Drop for Thing {
    fn drop(&mut self) {
        println!("drop for {}", self.name);
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} dividend divisor", args[0]);
        std::process::exit(1);
    }

    let a = Thing { name: String::from("first") };
    let b = Thing { name: String::from("second") };

    let dividend = args[1].parse::<f64>().unwrap();
    let divisor = args[2].parse::<f64>().unwrap();

    println!("{} / {}  =  {}", dividend, divisor, dividend/divisor);

    if dividend == 42.0 {
        let c = Thing { name: String::from("apple") };
        println!("after the apple was created");
        let c = Thing { name: String::from("banana") };
        println!("after the banana was created");
    }

    if divisor == 42.0 {
        let mut d = Thing { name: String::from("cat") };
        println!("after the cat was created");
        d = Thing { name: String::from("dog") };
        println!("after the dog was created");
    }

}
