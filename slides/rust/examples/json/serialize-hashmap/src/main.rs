use std::collections::HashMap;

fn main() {
    let mut data_before = HashMap::new();
    data_before.insert(String::from("foo"), 23);
    data_before.insert(String::from("bar"), 42);
    println!("data_before:  {data_before:?}");

    // serialize
    let json_string = serde_json::to_string(&data_before).unwrap();
    println!("serialized:   {json_string}");

    // deserialize
    let data_after: HashMap<String, u32> = serde_json::from_str(&json_string).unwrap();
    println!("deserialized: {data_after:?}");
    assert_eq!(data_before, data_after);

    // doing the same using Turbofish
    let data_turbofish = serde_json::from_str::<HashMap<String, u32>>(&json_string).unwrap();
    println!("turbofish:    {data_turbofish:?}");
    assert_eq!(data_before, data_turbofish);
}
