#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    fname: String,
    lname: String,
}

fn main() {
    let mut people = [
        Person {
            fname: String::from("John"),
            lname: String::from("Lennon"),
        },
        Person {
            fname: String::from("Paul"),
            lname: String::from("McCartney"),
        },
    ];

    println!("{:#?}", people);

    people[0].fname = String::from("Jane");
    println!("{:#?}", people);
}
