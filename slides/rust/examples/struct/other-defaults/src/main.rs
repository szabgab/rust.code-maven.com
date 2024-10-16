#[derive(Debug)]
#[allow(dead_code)]
struct Something {
    name: String,
    number: i32,
}

impl Default for Something {
    fn default() -> Something {
        Something {
            name: String::from("Rust"),
            number: 19,
        }
    }
}

fn main() {
    let sg = Something {
        name: String::from("Foo Bar"),
        number: 42,
        //..Default::default()   // We can enable this or leave it out, does not matter.
    };
    println!("{:?}", sg);
    assert_eq!(sg.name, "Foo Bar");
    assert_eq!(sg.number, 42);

    let empty = Something {
        ..Default::default()
    };
    println!("{:?}", empty);
    assert_eq!(empty.name, "Rust");
    assert_eq!(empty.number, 19);

    let with_name = Something {
        name: String::from("Hello"),
        ..Default::default()
    };
    println!("{:?}", with_name);
    assert_eq!(with_name.name, "Hello");
    assert_eq!(with_name.number, 19);

    let with_number = Something {
        number: 42,
        ..Default::default()
    };
    println!("{:?}", with_number);
    assert_eq!(with_number.name, "Rust");
    assert_eq!(with_number.number, 42);
}
