use std::cmp::Ordering;

#[allow(dead_code)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.height.cmp(&other.height))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

fn main() {
    let a = Person {
        id: 1,
        name: String::from("Foo"),
        height: 160,
    };

    let b = Person {
        id: 1,
        name: String::from("Foo"),
        height: 180,
    };

    let c = Person {
        id: 1,
        name: String::from("Foo"),
        height: 160,
    };

    let x = Person {
        id: 2,
        name: String::from("Bar"),
        height: 180,
    };

    println!("{}", a < b);
    println!("{}", a == c);
    println!("{}", b == x);
}
