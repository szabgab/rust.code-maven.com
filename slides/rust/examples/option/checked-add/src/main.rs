fn main() {
    nulled_area(20, 30);
}

fn nulled_area(width: u8, length: u8) -> u8 {
    let area_option = width.checked_mul(length);
    //println!("{:?}", area_option);
    match area_option {
        Some(area) => area,
        None => 0,
    }
}

fn checked_area(width: u8, length: u8) -> Option<u8> {
    width.checked_mul(length)
    //    let area_option = width.checked_mul(length);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nulled_area() {
        assert_eq!(nulled_area(3, 4), 12);
        assert_eq!(nulled_area(20, 20), 0);
    }

    #[test]
    fn test_checked_area() {
        assert_eq!(checked_area(3, 4), Some(12));
        assert_eq!(checked_area(20, 20), None);
    }
}
