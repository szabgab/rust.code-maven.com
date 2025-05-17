use core::panic;
use std::collections::HashMap;

fn main() {
    let filename = "/usr/share/dict/american-english";
    let content = std::fs::read_to_string(filename).unwrap();
    let rows = content.lines().collect::<Vec<_>>();
    //let rows = ("a b c d e f g h i j k l m n o p q r s t u v w x y z").split_whitespace().collect::<Vec<_>>();
    println!("Total words: {}", rows.len());

    // disregard empty strings
    let mut first_letter: HashMap<String, usize> = HashMap::new();
    for row in &rows {
        if row.len() < 1 {
            continue;
        }
        let first = row.chars().next().unwrap().to_string();
        let count = first_letter.entry(first).or_insert(0);
        *count += 1;
    }
    //println!("First letter count: {:?}", first_letter);
    //println!("First letter keys: {:?}", first_letter.keys());
    let bucket_count = first_letter.keys().len();
    println!("Number of buckets: {}", bucket_count);
    let expected = rows.len() as f64 / bucket_count as f64;
    println!("chi_square: {}", chi_square(&first_letter, expected));

    // disregard any string that has only one letter
    let mut two_letters: HashMap<String, usize> = HashMap::new();
    for row in &rows {
        if row.len() < 2 {
            continue;
        }
        let mut chars = row.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        let key = format!("{}{}", first, second);
        let count = two_letters.entry(key).or_insert(0);
        *count += 1;
    }

    //println!("Two letters count: {:?}", two_letters);
    let bucket_count = two_letters.keys().len();
    println!("Number of buckets: {}", bucket_count);
    let expected = rows.len() as f64 / bucket_count as f64;
    println!("chi_square: {}", chi_square(&two_letters, expected));
}

fn chi_square(observed: &HashMap<String, usize>, expected: f64) -> f64 {
    let mut chi_square = 0.0;
    if expected == 0.0 {
        panic!("Expected count should not be zero");
    }
    for (_, &observed_count) in observed {
        let diff = observed_count as f64 - expected;
        chi_square += diff * diff / expected;
    }
    chi_square
}
