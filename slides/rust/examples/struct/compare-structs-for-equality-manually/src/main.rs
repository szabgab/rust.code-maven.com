#[allow(dead_code)]
struct Thing {
    name: String,
    number: i32,
}

impl PartialEq for Thing {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

fn main() {
    let a = Thing {
        name: String::from("Foo"),
        number: 42,
    };

    let b = Thing {
        name: String::from("Foo"),
        number: 42,
    };

    let c = Thing {
        name: String::from("Foo1"),
        number: 42,
    };

    println!("{}", a == b);
    println!("{}", a == c);

    // We cannot compare which is bigger as we have not implemented
    // (or derived from) Ord or PartialOrd.
    // println!("{}", a < c);
}

