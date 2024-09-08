pub fn area(width: u64, length: u64) -> u64 {
    width * length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = area(2, 3);
        assert_eq!(result, 6);
    }
}
