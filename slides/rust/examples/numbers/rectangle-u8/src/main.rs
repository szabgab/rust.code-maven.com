fn main() {
}

fn area(width: u8, length: u8) -> u8 {
    let area = width * length;
    area
}

#![cfg](test)
mod test {

    fn test_area() {
        assert_eq!(area(2, 3), 6);
    }
}

