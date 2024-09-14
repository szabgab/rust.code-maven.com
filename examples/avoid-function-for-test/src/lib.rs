
pub fn diff(before: &str, after: &str) -> i64 {
    let before_temperature = get_temp(before);
    let after_temperature = get_temp(after);

    after_temperature - before_temperature
}

#[cfg(not(test))]
pub fn get_temp(when: &str) -> i64 {
    // This is an external API call that takes a long time
    std::thread::sleep(std::time::Duration::from_secs(2));
    match when {
        "2024.01.01" => 30,
        "2024.01.02" => 25,
        "2024.01.03" => 26,
        "2024.01.04" => 27,
        "2024.01.05" => 28,
        // ..
        _ => panic!("no such date"),
    }
}

#[cfg(test)]
pub fn get_temp(when: &str) -> i64 {
    match when {
        "2024.01.01" => 30,
        "2024.01.02" => 25,
        _ => panic!("no such date"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = diff("2024.01.01", "2024.01.02");
        assert_eq!(result, -5);


        // let result = diff("2024.01.02", "2024.01.03");
        // assert_eq!(result, 1);

    }
}
