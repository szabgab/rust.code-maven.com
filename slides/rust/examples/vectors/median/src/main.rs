fn main() {
    let numbers = vec![10, 12, 11];
    println!("{:?}", numbers);
    println!("{}", median(&numbers));
    println!("{:?}", numbers);

    let numbers = vec![10, 12, 13, 11];
    println!("{:?}", numbers);
    println!("{}", median(&numbers));
    println!("{:?}", numbers);
}

fn median(numbers: &[i32]) -> i32 {
    let mut numbers = numbers.to_owned();
    numbers.sort();
    let middle = numbers.len()/2;
    numbers[middle]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_median() {
        let numbers = vec![10, 12, 11];
        let med = median(&numbers);
        assert_eq!(med, 11);
        assert_eq!(numbers, vec![10, 12, 11]);

        let numbers = vec![10, 12, 13, 11];
        let med = median(&numbers);
        assert_eq!(med, 12);
        assert_eq!(numbers, vec![10, 12, 13, 11]);

    }

}
