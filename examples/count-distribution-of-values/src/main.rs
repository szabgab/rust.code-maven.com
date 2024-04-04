use std::collections::HashMap;

fn main() {
    let animals = vec![
        "camel", "snake", "camel", "snake", "crab", "snake", "crab", "crab", "crab",
    ];
    let counted = count_in_loop(&animals);
    println!("{:#?}", counted);
}

fn count_in_loop<'a>(words: &'a [&'a str]) -> HashMap<&'a str, u32> {
    let mut counter: HashMap<&str, u32> = HashMap::new();
    for word in words {
        *counter.entry(word).or_insert(0) += 1;
    }
    counter
}


#[test]
fn test_count() {
    let words = vec![
        "camel", "snake", "camel", "snake", "crab", "snake", "crab", "crab", "crab",
    ];
    let expected = HashMap::from([("camel", 2), ("crab", 4), ("snake", 3)]);
    assert_eq!(count_in_loop(&words), expected);
}
