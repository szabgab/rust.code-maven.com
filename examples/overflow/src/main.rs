fn main() {
    let mut count: u8 = 254;
    println!("count: {}", count);
    for _ in 1..=3 {
        count += 1;
        println!("count: {}", count);
    }

    let mut count: u8 = 254;
    println!("count: {}", count);
    for _ in 1..=3 {
        match count.checked_add(1) {
            Some(val) => count = val,
            None => eprintln!("Too much!"),
        };
        println!("count: {}", count);
    }

    let mut count: u8 = 254;
    println!("count: {}", count);
    for _ in 1..=3 {
        count = count.saturating_add(1);
        println!("count: {}", count);
    }

    let mut count: u8 = 254;
    println!("count: {}", count);
    for _ in 1..=3 {
        let (new_count, overflow) = count.overflowing_add(1);
        count = new_count;
        println!("count: {:3} {}", count, overflow);
    }

    let mut count: u8 = 254;
    println!("count: {}", count);
    for _ in 1..=3 {
        (count, _) = count.overflowing_add(1);
        println!("count: {}", count);
    }
}
