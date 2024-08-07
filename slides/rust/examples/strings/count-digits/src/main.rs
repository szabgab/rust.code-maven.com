fn main() {
    let text = "1213 456 78843978523 3224 2421";
    let mut counter = [0i8; 10];
    for ch in text.chars() {
        if ch == ' ' {
            continue;
        }
        let ix = ch as usize - '0' as usize;
        //println!("{ch}");
        //println!("{ix}");
        counter[ix] += 1;
    }
    for (ix, count) in counter.iter().enumerate() {
        println!("{ix}: {}", count);
    }
}
