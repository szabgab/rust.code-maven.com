fn main() {
    let numbers = vec![23, 12, 7 , 8];

    if numbers.iter().all(|num| *num > 0) {
        println!("Positive numbers");
    }
    if numbers.iter().all(|num| *num >= 10) {
        println!("Double digit numbers");
    }

    if numbers.into_iter().all(|num| num > 0) {
        println!("Positive numbers");
    }

    // use of moved value: `numbers`
    // if numbers.into_iter().all(|num| num >= 10) {
    //     println!("Double digit numbers");
    // }
}
