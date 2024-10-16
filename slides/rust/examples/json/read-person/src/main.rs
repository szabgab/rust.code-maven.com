use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Child {
    name: String,
    birthdate: u32,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Person {
    fname: String,
    lname: String,
    year: u32,
    height: f32,
    married: bool,
    numbers: Vec<u32>,
    children: Vec<Child>,
}

fn main() {
    let filename = get_filename();

    let content = std::fs::read_to_string(filename).unwrap();

    let person = serde_json::from_str::<Person>(&content).unwrap();
    println!("person = {:#?}", person);
    assert!(person.fname == "Foo");
    assert!(person.numbers[0] == 23);
    assert!(person.children[0].name == "Alpha");
}

fn get_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    args[1].to_owned()
}
