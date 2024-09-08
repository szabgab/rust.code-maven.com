#![allow(dead_code)]


struct Rectangle {
    width: u64,
    length: u64,
}

fn area(rect: &Rectangle) -> u64 {
    rect.width * rect.length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let r = Rectangle { width: 2, length: 3 };
        let result = area(&r);
        assert_eq!(result, 6);
    }
}
