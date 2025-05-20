use core::panic;
use std::collections::HashMap;

fn main() {
    let filename = "/usr/share/dict/american-english";
    let content = std::fs::read_to_string(filename).unwrap();
    let rows = content.lines().collect::<Vec<_>>();
    println!("Total words: {}", rows.len());

    println!("---\nOriginal:");
    measure(&rows, |row| row.to_string());

    println!("---\nFirst character:");
    measure(&rows, |row| {
        if row.is_empty() {
            String::new()
        } else {
            row.chars().next().unwrap().to_string()
        }
    });

    println!("---\nFirst two characters:");
    measure(&rows, |row| {
        if row.is_empty() {
            String::new()
        } else if row.len() == 1 {
            row.chars().next().unwrap().to_string()
        } else {
            let mut chars = row.chars();
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();
            format!("{}{}", first, second)
        }
    });

    println!("---\nFirst three characters:");
    measure(&rows, |row| {
        if row.is_empty() {
            return String::new();
        }

        let mut chars = row.chars();
        let first = chars.next().unwrap();

        if row.len() == 1 {
            return first.to_string();
        }

        let second = chars.next().unwrap();
        if row.len() == 2 {
            return format!("{}{}", first, second);
        }

        let third = chars.next().unwrap();
        format!("{}{}{}", first, second, third)
    });

    println!("---\nmd5:");
    measure(&rows, |row| {
        let digest = md5::compute(row);
        format!("{:x}", digest)
    });
}

fn measure<F>(rows: &Vec<&str>, key_extractor: F)
where
    F: Fn(&str) -> String,
{
    let mut counts: HashMap<String, usize> = HashMap::new();
    for row in rows {
        let key = key_extractor(row);
        let count = counts.entry(key).or_insert(0);
        *count += 1;
    }
    let bucket_count = counts.keys().len();
    println!("Number of buckets: {}", bucket_count);
    let expected = rows.len() as f64 / bucket_count as f64;
    println!("chi_square: {}", chi_square(&counts, expected));
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
