fn main() {
    let res = area(2, 3);
    assert_eq!(res, 6);
}

#[allow(clippy::let_and_return)]
fn area(width: u8, length: u8) -> u8 {
    let area = width.saturating_mul(length);
    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(area(2, 3), 6);
        assert_eq!(area(20, 20), 255);
    }
}
