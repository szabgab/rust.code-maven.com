#[derive(Debug, Default)]
#[allow(dead_code)]
struct Something {
    name: String,
    number: i32,
}

fn main() {
    let sg = Something {
        name: String::from("Foo Bar"),
        number: 42,
    };
    println!("{:?}", sg);
    assert_eq!(sg.name, "Foo Bar");
    assert_eq!(sg.number, 42);

    let empty = Something {
        ..Something::default()
    };
    println!("{:?}", empty);
    assert_eq!(empty.name, "");
    assert_eq!(empty.number, 0);

    let with_name = Something {
        name: String::from("Hello"),
        ..Something::default()
    };
    println!("{:?}", with_name);
    assert_eq!(with_name.name, "Hello");
    assert_eq!(with_name.number, 0);

    let with_number = Something {
        number: 42,
        ..Something::default()
    };
    println!("{:?}", with_number);
    assert_eq!(with_number.name, "");
    assert_eq!(with_number.number, 42);
}
