use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Thing {
    name: String,
    number: u32,
}


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Xhing<'a> {
    name: &'a str,
    number: u32,
}


fn main() {
    let thing = get_thing("Foo Bar");
    println!("{thing:#?}");


    let thing = get_thing_from_text_file("data.txt");
    println!("{thing:#?}");

    let thing = get_thing_from_json_file("data.json");
    println!("{thing:#?}");

    // let thing = get_xhing_from_json_file("data.json");
    // println!("{thing:#?}");

}


fn get_thing(name: &str) -> Thing {
    let thing = Thing {
        name: String::from(name),
        number: 42,
    };
    println!("{thing:#?}");

    thing
}

fn get_thing_from_text_file(path: &str) -> Thing {
    let raw = std::fs::read_to_string(path).unwrap();
    let (name, number) = raw.split_once(",").unwrap();
    Thing {
        name: name.to_string(),
        number: number.parse().unwrap(),
    }

}

fn get_thing_from_json_file(path: &str) -> Thing {
    let raw = std::fs::read_to_string(path).unwrap();
    serde_json::from_str::<Thing>(&raw).unwrap()
}


// fn get_xhing_from_json_file(path: &str) -> Xhing {
//     let raw = std::fs::read_to_string(path).unwrap();
//     serde_json::from_str::<Xhing>(&raw).unwrap()
// }

// fn return_str<'a>() -> &'a str {
//     "abc"   
// }