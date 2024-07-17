#[derive(PartialOrd, PartialEq)]
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
    let yesterday = Weekday::Friday;
    let today = Weekday::Saturday;
    let tomorrow = Weekday::Sunday;

    if yesterday < today {
        println!("Today is after yesterday");
    }
    if today < tomorrow {
        println!("Tomorrow is after today");
    }
}
