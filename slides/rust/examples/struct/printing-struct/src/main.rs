use std::fmt;

struct Animal<'a> {
    name: &'a str,
    size: &'a str,
    weight: i32,
}

impl std::fmt::Display for Animal<'_> {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(format, "name: {}, size: {}, weight: {}", self.name, self.size, self.weight)
    }
}


fn main() {
    let eli = Animal {name: "elephant", size: "huge", weight: 100};
    println!("{}", eli.name);
    println!("{}", eli.size);
    println!("{}", eli.weight);


    println!("{}", eli);
}

