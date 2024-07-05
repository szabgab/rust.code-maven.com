fn main() {
    println!("{}", do_something("42"));
    println!("{}", do_something("hello"));
}

fn do_something(input: &str) -> String {
    let value = match input.parse::<u32>() {
        Ok(val) => val,
        Err(err) => return format!("Error: {err}"),
    };
    // processing value
    format!("{value}")
}    


// use std::time::{SystemTime, UNIX_EPOCH};

// fn main() {
//     // let report = do_some_work();
//     // println!("{report}");

//     let report = flatten();
//     println!("{report}");

// }


// fn do_some_work() -> String {
//     match get_number() {
//         Ok(number) => match check(number) {
//             Ok(number) => format!("Number: {number}"),
//             Err(err) =>  {
//                 eprintln!("{err}");
//                 format!("Error: {err}")
//             }
//         },
//         Err(err) => {
//             eprintln!("{err}");
//             format!("Error: {err}")
//         }
//     }
// }

// fn flatten() -> String {
//     //flatten2().map_or_else(|err| format!("Error: {err}"))

//     match flatten2() {
//         Ok(number) => format!("Number: {number}"),
//         Err(err) => format!("Error: {err}"),
//     }
// }

// fn flatten2() -> Result<u128, &'static str> {
//     let number = get_number()?;
//     let number = check(number)?;
//     Ok(number)
// }



// fn check(number: u128) -> Result<u128, &'static str> {
//     if number % 3 == 0 {
//         Ok(number)
//     } else {
//         Err("Invalid number")
//     }
// }


// fn get_number() -> Result<u128, &'static str> {
//     let start = SystemTime::now();
//     let since_the_epoch = start
//     .duration_since(UNIX_EPOCH)
//     .expect("Time went backwards");

//     let time = since_the_epoch.as_micros();
//     if time % 2 == 0 {
//         Ok(time)
//     } else {
//         Err("Could not get good number")
//     }
// }


// fn main() {
//     assert_eq!(guess("23"), Some(false));
//     assert_eq!(guess("42"), Some(true));
//     assert_eq!(guess("abc"), None);
// }



// fn guess(input: &str) -> Option<bool> {
//     match input.parse::<u32>() {
//         Ok(number) => Some(number == 42),
//         Err(err) => {
//             eprintln!("{err}");
//             None
//         }
//     }
// }

// fn guess(input: &str) -> Option<bool> {
//     let number = match input.parse::<u32>() {
//         Ok(number) => number,
//         Err(err) => {
//             eprintln!("{err}");
//             return None;
//         }
//     };

//     Some(number == 42)
// }

// fn guess(input: &str) -> Option<bool> {
//     let number = input.parse::<u32>().ok()?;

//     Some(number == 42)
// }



