use std::collections::HashMap;

fn main() {
    let mut things: HashMap<(&str, &str), &str> = HashMap::new();

    let entries= vec!["apple,red,23", "banana,green,3", "apple,red,42"];

    for entry in entries {
        let parts = entry.split(',').collect::<Vec<&str>>();
        let key = (parts[0], parts[1]);
        let value = parts[2];
        //println!("{:?}   {}", key, value);
        things.insert(key, value);

        println!("{:#?}", things);
    }

}
