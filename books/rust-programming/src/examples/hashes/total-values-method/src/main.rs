use std::collections::HashMap;

pub trait SumValues<'a> {
    fn add(&mut self, other: &HashMap<&'a str, i32>);
}

impl<'a> SumValues<'a> for HashMap<&'a str, i32> {
    fn add(&mut self, other: &HashMap<&'a str, i32>) {
        // println!("self: {self:?}");
        // println!("other: {other:?}");
        for (key, value) in other.iter() {
            *self.entry(key).or_insert(0) += value;
        }
        // println!("self: {self:?}");
    }
}

fn main() {
    let a = HashMap::from([("apple", 1), ("banana", 1)]);
    let b = HashMap::from([("apple", 2), ("peach", 2), ("grape", 2)]);

    let mut total: HashMap<&str, i32> = HashMap::new();

    total.add(&a);
    println!("{:#?}", total);
    assert_eq!(total, a);

    total.add(&b);
    println!("{:#?}", total);
    assert_eq!(
        total,
        HashMap::from([("apple", 3), ("peach", 2), ("grape", 2), ("banana", 1),])
    );
}
