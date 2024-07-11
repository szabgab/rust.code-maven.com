use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Person {
    fname: String,
    lname: String,
    married: bool,
}


fn main() {
    let filename = get_filename();

    let content = std::fs::read_to_string(filename).unwrap();
    let data = serde_json::from_str::<Person>(&content).expect("JSON parsing error");
    println!("{:#?}", &data);
    assert_eq!(data.fname, "Foo");
    assert_eq!(data.lname, "Bar");
    assert!(data.married);
}


fn get_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    args[1].to_owned()
}

