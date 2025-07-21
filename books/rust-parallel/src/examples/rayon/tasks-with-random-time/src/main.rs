//use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    //let numbers = (1..1000).map(|_| rand::thread_rng().gen_range(0..=300) as u64).collect::<Vec<_>>();
    let mut numbers = vec![3000_u64, 3000];
    numbers.extend([1; 1000]);

    // println!("{numbers:?}");

    let parallel = numbers
        .par_iter()
        .map(|number| {
            //println!("{:?} sleeping for {}", std::thread::current().id(), number);
            std::thread::sleep(std::time::Duration::from_millis(*number));
            format!("{:?}", std::thread::current().id())
        })
        .collect::<Vec<_>>();

    // println!("{parallel:?}");

    let mut counter: HashMap<String, u32> = HashMap::new();
    for id in parallel {
        *counter.entry(id).or_insert(0) += 1;
    }
    for (key, value) in counter {
        println!("{key} {value}");
    }
}
