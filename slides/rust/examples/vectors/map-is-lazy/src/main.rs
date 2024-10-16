fn main() {
    let numbers = vec![1, 3, 6];
    let double = numbers.into_iter().map(|num| {
        println!("doubling {num}");
        num * 2
    });

    println!("Nothing happended yet");
    for num in double {
        println!("{num:?}");
    }
}
