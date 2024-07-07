
use std::collections::VecDeque;

fn main() {

    let mut doctor = VecDeque::new();
    println!("len: {} capacity: {} empty:  {}", doctor.len(), doctor.capacity(), doctor.is_empty());

    doctor.push_back(String::from("cat"));
    println!("len: {} capacity: {} empty:  {}", doctor.len(), doctor.capacity(), doctor.is_empty());

    doctor.push_back(String::from("dog"));
    doctor.push_back(String::from("rat"));
    doctor.push_back(String::from("squirrel"));
    doctor.push_back(String::from("snake"));
    println!("len: {} capacity: {} empty:  {}", doctor.len(), doctor.capacity(), doctor.is_empty());

    println!("{:?}", doctor); // I can print the whole queue, but there is probably no point for that

    let next = doctor.pop_front();
    println!("{}", next.unwrap());
    println!("len: {} capacity: {} empty:  {}", doctor.len(), doctor.capacity(), doctor.is_empty());

}
