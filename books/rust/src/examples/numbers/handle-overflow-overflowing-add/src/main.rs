fn main() {
    let mut num: i8 = 126;
    println!("{num}");

    let mut ok;

    (num, ok) = num.overflowing_add(1);
    println!("{num} {ok}");

    (num, ok) = num.overflowing_add(1);
    println!("{num} {ok}");
}
