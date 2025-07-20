fn main() {
    println!("{}", repeat("a", 3));
}

fn repeat(txt: &str, n: u32) -> String {
    let mut text = String::new();
    println!("{:?}", n);
    for _ in 1..=n {
        text.push_str(txt);
    }

    text
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_repeat() {
        assert_eq!(repeat("1bc_", 3), "1bc_1bc_1bc_");
        // assert_eq!(repeat("2bc_", 3u8), "2bc_2bc_2bc_");
        // assert_eq!(repeat("3bc_", 3u16), "3bc_3bc_3bc_");
    }
}
