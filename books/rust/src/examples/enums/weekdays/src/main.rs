#[derive(Debug)]
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
    let today = Weekday::Sunday;

    println!("{:?}", today);
    println!();
    for day in [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Saturday,
        Weekday::Sunday,
    ] {
        println!("{:?}", day);
        match day {
            Weekday::Sunday => println!("Today is {day:?}, it is a day off in Europe"),
            Weekday::Saturday => println!("Today is {day:?}, it is a day off in Israel"),
            _ => println!("Today is {day:?}, it is a workday"),
        }
    }
}
