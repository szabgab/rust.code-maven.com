fn main() {
    let numbers = vec![1, 2, 3];
    println!("{:?}", numbers);
    let doubles: Vec<i32> = numbers.into_iter().map(|x| x + 1).collect();

    // println!("{:?}", numbers); // borrow of moved value: `numbers`
    // Using iter() instead of into_iter() would let us use the old vector as well.
    println!("{:?}", doubles);
}
