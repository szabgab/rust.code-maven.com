#[allow(dead_code)]
#[derive(Debug)]
enum Title {
    Doctor,
    Prof,
}

#[allow(dead_code)]
#[derive(Debug)]
enum PhoneType {
    Home,
    Work,
    Mobile,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Phone {
    number: String,
    ptype: PhoneType,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    id: String,
    phones: Vec<Phone>,
    notes: Option<String>,
    title: Option<Title>,
}

fn main() {
    let joe = Person {
        name: String::from("Joe"),
        id: String::from("123456"),
        phones: vec![Phone {
            number: String::from("055-1234567"),
            ptype: PhoneType::Home,
        }],
        notes: None,
        title: None,
    };

    let jane = Person {
        name: String::from("Jane"),
        id: String::from("678"),
        phones: vec![Phone {
            number: String::from("123"),
            ptype: PhoneType::Work,
        }],
        notes: Some(String::from("was a student")),
        title: Some(Title::Doctor),
    };

    println!("{:#?}", joe);
    println!("{:#?}", jane);
}
