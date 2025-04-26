use tracing::debug;

pub fn add(left: u64, right: u64) -> u64 {
    debug!(left, right, "add");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
