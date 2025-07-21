use std::sync;

fn main() {
    integer();
    string();
    vector();
}

fn integer() {
    let number = sync::Mutex::new(12);

    {
        if let Ok(mut guarded_number) = number.lock() {
            *guarded_number += 23;
        }
    }

    if let Ok(value) = number.into_inner() {
        println!("{value}");
    }
}

fn string() {
    let text = sync::Mutex::new(String::new());

    {
        let other_text = String::from("The black cat");
        if let Ok(mut guarded_text) = text.lock() {
            guarded_text.push_str(&other_text);
        }
    }

    if let Ok(value) = text.into_inner() {
        println!("{value}");
    }
}

fn vector() {
    let animals = sync::Mutex::new(vec![]);

    {
        if let Ok(mut guarded_animals) = animals.lock() {
            guarded_animals.push(String::from("tiger"));
        }
    }

    if let Ok(value) = animals.into_inner() {
        println!("{value:?}");
    }
}
