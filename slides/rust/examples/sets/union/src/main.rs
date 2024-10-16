use std::collections::HashSet;

fn main() {
    let mut english: HashSet<&str> = HashSet::new();
    let mut spanish: HashSet<&str> = HashSet::new();

    for word in ["door", "car", "lunar", "era"] {
        english.insert(word);
    }
    for word in ["era", "lunar", "hola"] {
        spanish.insert(word);
    }

    println!("{:?}", &english);
    println!("{:?}", &spanish);

    assert_eq!(english.len(), 4);
    assert_eq!(spanish.len(), 3);

    assert!(english.contains("door"));
    assert!(english.contains("era"));
    assert!(!english.contains("hola"));

    assert!(spanish.contains("hola"));

    let both_by_bitor = &english | &spanish;
    println!("{:?}", both_by_bitor);

    assert!(both_by_bitor.contains("door"));
    assert!(both_by_bitor.contains("era"));
    assert!(both_by_bitor.contains("hola"));

    // This does not change the original sets
    assert!(english.contains("door"));
    assert!(english.contains("era"));
    assert!(!english.contains("hola"));

    // Returns a https://doc.rust-lang.org/std/collections/hash_set/struct.Union.html that could be
    // collect-ed
    let both = &english.union(&spanish);
    println!("{:?}", both);
}
