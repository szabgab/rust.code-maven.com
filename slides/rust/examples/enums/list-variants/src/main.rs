use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
enum Animal {
    Cat,
    Dog,
    Snake,
    Camel,
    Crab,
}

fn main() {
    for animal in Animal::iter() {
        println!("{animal:?}");
    }
}
