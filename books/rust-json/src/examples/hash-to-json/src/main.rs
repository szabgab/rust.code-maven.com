use std::collections::HashMap;

fn main() {
    let mut data_before = HashMap::new();
    data_before.insert("foo".to_string(), 23);
    data_before.insert("bar".to_string(), 42);
    println!("{:?}", data_before);

    // serialize
    let json_string = serde_json::to_string(&data_before).unwrap();
    println!("{}", json_string);

    // deserialize
    let data_after: HashMap<String, u32> = serde_json::from_str(&json_string).unwrap();
    println!("{:?}", data_after);
    assert_eq!(data_before, data_after);

    // doing the same using Turbofish
    let data_turbofish = serde_json::from_str::<HashMap<String, u32>>(&json_string).unwrap();
    println!("{:?}", data_turbofish);
    assert_eq!(data_before, data_turbofish);

}
