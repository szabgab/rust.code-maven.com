use rayon::prelude::*;

fn main() {
    let numbers = (1..10).collect::<Vec<_>>();

    let linear = numbers.iter().map(|number| number * 2).collect::<Vec<_>>();
    let parallel = numbers
        .par_iter()
        .map(|number| number * 2)
        .collect::<Vec<_>>();

    println!("{numbers:?}");
    println!("{linear:?}");
    println!("{parallel:?}");
    assert_eq!(linear, parallel);

    let _parallel = numbers
        .par_iter()
        .map(|number| {
            println!("{:?}", std::thread::current().id());
            number * 2
        })
        .collect::<Vec<_>>();
}
