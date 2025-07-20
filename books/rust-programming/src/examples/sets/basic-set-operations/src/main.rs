use std::collections::HashSet;

fn main() {
    let mut english: HashSet<String> = HashSet::new();
    println!("{:?}", &english);
    assert_eq!(english.len(), 0);
    assert_eq!(format!("{:?}", &english), "{}");

    english.insert(String::from("chair"));
    println!("{:?}", &english);
    assert_eq!(english.len(), 1);
    assert_eq!(format!("{:?}", &english), r#"{"chair"}"#);

    english.insert(String::from("table"));
    println!("{:?}", &english);
    assert_eq!(english.len(), 2);
    assert!(
        format!("{:?}", &english) == r#"{"table", "chair"}"#
            || format!("{:?}", &english) == r#"{"chair", "table"}"#
    );

    english.insert(String::from("chair"));
    println!("{:?}", &english);
    assert_eq!(english.len(), 2);
    assert!(
        format!("{:?}", &english) == r#"{"table", "chair"}"#
            || format!("{:?}", &english) == r#"{"chair", "table"}"#
    );

    assert!(english.contains("chair"));
    assert!(!english.contains("door"));
    assert_eq!(english.len(), 2);

    println!("----");
    for word in &english {
        println!("{}", word);
    }
    println!("----");

    english.remove("chair");
    println!("{:?}", &english);
    assert_eq!(english.len(), 1);
    assert_eq!(format!("{:?}", &english), r#"{"table"}"#);
}
