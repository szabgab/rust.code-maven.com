fn main() {
    let taken = (0..).skip(5).take(7).collect::<Vec<_>>();
    let range = (5..12).collect::<Vec<_>>();
    assert_eq!(taken, range);
    println!("{:?}", taken);
}
