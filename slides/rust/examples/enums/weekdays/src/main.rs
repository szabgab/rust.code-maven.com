#[derive(PartialEq, Debug)]
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

fn main() {
    let today = Weekday::Sunday;
    let other_day = Weekday::Monday;
    println!("{:?}", today == other_day);

    println!("{:?}", today);
    println!();
    for day in [Weekday::Monday, Weekday::Tuesday, Weekday::Saturday, Weekday::Sunday] {
        println!("{:?}", day);
        match day {
            Weekday::Sunday => println!("Today is {day:?}, it is a day off in Europe"),
            Weekday::Saturday => println!("Today is {day:?}, it is a day off in Israel"),
            _ => println!("Today is {day:?}, it is a workday"),
        }
    }
}

