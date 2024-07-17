use std::fmt;

struct Animal {
    name: String,
    size: String,
    weight: i32,
}

impl std::fmt::Debug for Animal {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "Animal {{ name: {}, size: {}, weight: {} }}", self.name, self.size, self.weight)
    }
}

fn main() {
    let eli = Animal {name: String::from("elephant"), size: String::from("huge"), weight: 100};
    println!("{}", eli.name);
    println!("{}", eli.size);
    println!("{}", eli.weight);


    println!("{:?}", eli);
    dbg!(eli);
}


