pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result = add(3, 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn it_works_1_2() {
        test_add(1, 2, 3)
    }
    #[test]
    fn it_works_2_3() {
        test_add(2, 3, 5)
    }

    fn test_add(a: u64, b: u64, expected: u64) {
        let result = add(a, b);
        assert_eq!(result, expected);
    }
}
