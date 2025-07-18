#![allow(clippy::derivable_impls)]

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
        //..Default::default()
    };
    println!("{:?}", sg);
    assert_eq!(sg.name, "Foo Bar");
    assert_eq!(sg.number, 42);

    let empty = Something {
        ..Default::default()
    };
    println!("{:?}", empty);
    assert_eq!(empty.name, "");
    assert_eq!(empty.number, 0);

    let with_name = Something {
        name: String::from("Hello"),
        ..Default::default()
    };
    println!("{:?}", with_name);
    assert_eq!(with_name.name, "Hello");
    assert_eq!(with_name.number, 0);

    let with_number = Something {
        number: 42,
        ..Default::default()
    };
    println!("{:?}", with_number);
    assert_eq!(with_number.name, "");
    assert_eq!(with_number.number, 42);
}
