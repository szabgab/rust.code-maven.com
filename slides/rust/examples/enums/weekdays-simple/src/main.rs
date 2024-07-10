enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// We need the allow dead_code beacause in this example we did not use all the values that were listed in the enum

fn handle_day(day: Weekday) {
    match day {
        Weekday::Monday => println!("Today is Monday"),
        Weekday::Tuesday => println!("Today is Tuesday"),
        Weekday::Wednesday => println!("Today is Wednesday"),
        Weekday::Thursday => println!("Today is Thursday"),
        Weekday::Friday => println!("Today is Friday"),
        Weekday::Saturday => println!("Today is Saturday"),
        Weekday::Sunday => println!("Today is Sunday"),
    }
}

fn main() {
    handle_day(Weekday::Sunday);

    let sat = Weekday::Saturday;
    handle_day(sat);

    let weekdays = [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
    ];

    for day in weekdays {
        handle_day(day);
    }
}
