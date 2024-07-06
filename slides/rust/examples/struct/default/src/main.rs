
#[derive(Debug)]
#[allow(dead_code)]
struct Something {
    name: String,
    number: i32,
}

impl Default for Something {
    fn default() -> Something {
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
    dbg!(sg);

    let empty = Something {
        ..Something::default()
    };
    dbg!(empty);

    let with_name = Something {
        name: String::from("Hello"),
        ..Something::default()
    };
    dbg!(with_name);

    let with_number = Something {
        number: 42,
        ..Something::default()
    };
    dbg!(with_number);

}
