pub fn div(left: u64, right: u64) -> u64 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;


    struct Thing {
        name: String,
    }

    impl Thing {
        pub fn new(name: &str) -> Self {
            println!("before {name}");
            Self {
                name: name.to_owned(),
            }
        }
    }

    impl Drop for Thing {
        fn drop(&mut self) {
            println!("after  {}", self.name);
        }
    }

    #[test]
    fn works_2_2() {
        let _thing = Thing::new("2_2");
        let result = div(2, 2);
        assert_eq!(result, 1);
    }


    #[test]
    fn works_2_0() {
        let _thing = Thing::new("2_0");
        let result = div(2, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn works_4_3() {
        let _thing = Thing::new("4_3");
        let result = div(4, 3);
        assert_eq!(result, 1);
    }

}
