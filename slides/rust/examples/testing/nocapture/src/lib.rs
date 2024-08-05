pub fn add(left: usize, right: usize) -> usize {
    println!("STDOUT In the application");
    eprintln!("STDERR In the application");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("STDOUT in the test");
        eprintln!("STDERR in the test");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
