#![allow(dead_code)]


struct Rectangle {
    width: u64,
    length: u64,
}

pub trait Area {
    fn area(&self) -> u64;
}

impl Area for Rectangle {
    fn area(&self) -> u64 {
        self.long();
        self.width * self.length
    }
}

pub trait Long {
    fn long(&self);
}

impl Long for Rectangle {
    fn long(&self) {
        std::thread::sleep(std::time::Duration::from_secs(20));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let r = Rectangle { width: 2, length: 3 };
        let result = r.area();
        assert_eq!(result, 6);
    }
}
