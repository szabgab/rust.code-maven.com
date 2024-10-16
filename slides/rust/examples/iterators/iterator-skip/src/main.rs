fn main() {
    let skipped = (0..10).skip(5).collect::<Vec<_>>();
    let range = (5..10).collect::<Vec<_>>();
    assert_eq!(skipped, range);
    println!("{:?}", skipped);

    // If we exhaust the iterator while skipping, that's fine. We get an empty iterator.
    let numbers = (0..5).skip(10).collect::<Vec<_>>();
    println!("{:?}", numbers);
}
