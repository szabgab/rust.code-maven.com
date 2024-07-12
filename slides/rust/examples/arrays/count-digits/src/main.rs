fn main() {
    let digits = [1, 2, 1, 3, 4, 5, 6, 7, 8, 8, 4, 3, 9, 7, 8, 5, 2, 3, 3, 2, 2, 4, 2, 4, 2, 1];
    let mut counter = [0i8; 10];
    for ch in digits {
        counter[ch] += 1;
    }
    for (ix, count) in counter.iter().enumerate() {
        println!("{ix}: {}", count);
    }
}
