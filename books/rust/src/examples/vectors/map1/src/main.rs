fn main() {
    let chars = vec!['a', 'b', 'c'];
    let mut cnt = 0;
    let pairs = chars.into_iter().map(|letter| {
        cnt += 1;
        (letter, cnt)
    });
    for pair in pairs {
        println!("{pair:?}");
    }
}
