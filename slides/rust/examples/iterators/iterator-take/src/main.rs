fn main() {
    let n = 5;
    let taken = (0..).take(n).collect::<Vec<_>>();
    let range = (0..n).collect::<Vec<_>>();
    assert_eq!(taken, range);
    println!("{:?}", taken);

    // If there are not enough iterations it stops when the iterator is exhausted
    let numbers = range.iter().take(10).collect::<Vec<_>>();
    println!("{:?}", numbers);
}
