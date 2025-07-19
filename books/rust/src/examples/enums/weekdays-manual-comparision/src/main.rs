#[derive(PartialEq)]
#[allow(dead_code)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// We need the allow dead_code beacause in this example
// we did not use all the values that were listed in the enum

fn main() {
    let today = Weekday::Saturday;
    let tomorrow = Weekday::Sunday;
    let market = Weekday::Sunday;

    if market == today {
        println!("Today is market day");
    }
    if market == tomorrow {
        println!("Tomorrow is market day");
    }
}
