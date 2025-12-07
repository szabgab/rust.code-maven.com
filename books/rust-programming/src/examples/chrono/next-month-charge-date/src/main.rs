use chrono::{Datelike, NaiveDate, Utc};

fn main() {
    let now = Utc::now().date_naive();
    let next_charge = next_charge_date(now);
    println!("Original date:    {}", now);
    println!("Next charge date: {}", next_charge);

    test_edge_cases();
}

// month + 1
// If the days is 29, 30, or 31, set the next to month + 1  and the day to 1 (we give a few days present to those who sign up on dates that are not in every month)
// if the month is > 12, set month to 1 and year + 1
fn next_charge_date(date: NaiveDate) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month();
    let mut day = date.day();

    month += 1;
    if day > 28 {
        month += 1;
        day = 1;
    }

    if month > 12 {
        month -= 12;
        year += 1;
    }

    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

fn test_edge_cases() {
    let test_cases = vec![
        [
            NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 10).unwrap(),
        ],
        [
            NaiveDate::from_ymd_opt(2024, 1, 29).unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        ],
        [
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        ],
        [
            NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(),
        ],
        [
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(),
        ],
    ];

    for case in test_cases {
        assert!(next_charge_date(case[0]) == case[1]);
    }
}
