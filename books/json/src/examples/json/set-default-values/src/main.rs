use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Person {
    #[serde(default = "get_default_fname")]
    fname: String,

    #[serde(default = "get_default_false")]
    married: bool,
}

fn get_default_fname() -> String {
    String::from("Foo")
}

fn get_default_false() -> bool {
    false
}

fn main() {
    let content = "{}";
    let data = serde_json::from_str::<Person>(content).expect("JSON parsing error");
    println!("{:#?}", &data);
}
