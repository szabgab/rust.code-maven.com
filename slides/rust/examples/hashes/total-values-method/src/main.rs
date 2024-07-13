use std::collections::HashMap;

// pub trait SumValues<'a> {
//     fn add(&'a mut self, other : &HashMap<&'a str, i32>);
// } 

// impl<'a> SumValues<'a> for HashMap<&'a str, i32> {
//     fn add(&'a mut self, other : &HashMap<&'a str, i32>) {
//         for (key, value) in other.iter() {
//             *self.entry(key).or_insert(0) += value;
//         }
//     }
// }

pub trait SumValues<'a> {
    fn add(self, other : &HashMap<&'a str, i32>);
} 

impl<'a> SumValues<'a> for HashMap<&'a str, i32> {
    fn add(mut self, other : &HashMap<&'a str, i32>) {
        for (key, value) in other.iter() {
            *self.entry(key).or_insert(0) += value;
        }
    }
}

fn main() {
    let a = HashMap::from([
        ("apple", 1),
        ("banana", 1),
    ]);
    // let b = HashMap::from([
    //     ("apple", 2),
    //     ("peach", 2),
    //     ("grape", 2),
    // ]);

    let mut total: HashMap<&str, i32> = HashMap::new();

    total.add(&a);

    // for (k, v) in &total.iter() {
    //     println!("{k} => {v}");
    // }
    println!("{:#?}", total);

    // add(&mut total, &b);
    // println!("{:#?}", total);
}

