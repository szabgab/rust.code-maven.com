
fn main() {
    let numbers = (1..100).collect::<Vec<u32>>();
    println!("total: {}", numbers.len());

    let small = numbers.iter().filter(|num| **num < 10).collect::<Vec<_>>();
    println!("small: {}", small.len());

    let big = numbers.iter().filter(|num| **num > 80).collect::<Vec<_>>();
    println!("big:   {}", big.len());

    let odd = numbers.iter().filter(|num| *num % 2 == 1).collect::<Vec<_>>();
    println!("odd:   {}", odd.len());
    println!("odd:   {:?}", odd);
}
