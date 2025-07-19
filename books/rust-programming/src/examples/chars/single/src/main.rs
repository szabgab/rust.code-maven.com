fn main() {
    let chars = ['1', 'a', 'א', 'Ω', '😇', '😈'];
    for ch in chars {
        let usize_num = ch as usize;
        let u32_num = ch as u32;

        let back = char::from_u32(u32_num).expect("Could not convert to char");

        println!(
            "{ch}\t{usize_num:6} {u32_num:6} {back} \t{} {}",
            (ch == back),
            ch.len_utf8()
        );
    }
}
