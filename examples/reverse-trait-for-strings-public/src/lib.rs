pub trait Reverse {
    fn reverse(&self) -> String;
}

impl Reverse for str {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Hello";
        assert_eq!("olleH", text.reverse());

        let text = "mañana";
        assert_eq!("anañam", text.reverse());
    }
}
