use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
struct Student {
    name: String,
    grades: HashMap<String, u8>,
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

fn main() {
    compare_numbers();
    compare_points();
    compare_students();
}

fn compare_numbers() {
    let a = 10;
    let b = 20;

    let result = if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    };

    println!("Comparison result: {}", result);
}

fn compare_points() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5, y: 15 };

    let result = if p1 > p2 {
        1
    } else if p1 < p2 {
        -1
    } else {
        0
    };

    println!("Comparison result: {}", result);
}

fn compare_students() {
    let s1 = Student {
        name: String::from("Alice"),
        grades: HashMap::from([
            (String::from("Math"), 100),
            (String::from("Literature"), 80),
        ]),
    };

    let s2 = Student {
        name: String::from("Bob"),
        grades: HashMap::from([(String::from("Math"), 90), (String::from("Literature"), 91)]),
    };

    let result = if s1 > s2 {
        1
    } else if s1 < s2 {
        -1
    } else {
        0
    };

    println!("Comparison result: {}", result);
}
