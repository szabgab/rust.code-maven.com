fn main() {
    let res = add(19, 23);
    println!("Result: {res}");
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result = add(19, 23);
        assert_eq!(result, 42);
    }
}
