#[derive(Debug)]
#[allow(dead_code)]
struct Something {
    name: String,
    number: i32,
}

impl Something {
    pub fn new() -> Something {
        Something {
            name: String::new(),
            number: 0,
        }
    }
}

fn main() {
    let sg = Something {
        name: String::from("Foo Bar"),
        number: 42,
    };
    println!("{:?}", sg);

    let new = Something::new();
    println!("{:?}", new);
}
