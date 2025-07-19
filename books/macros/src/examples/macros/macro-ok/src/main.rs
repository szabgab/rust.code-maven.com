macro_rules! ok(($result:expr) => ($result.unwrap()));

//macro_rules! ok{($result:expr) => ($result.unwrap())}

// macro_rules! ok {
//     ($result:expr) => {
//         $result.unwrap()
//     }
// }

fn main() {
    for input in ["42", "4.2", "23"] {
        let number = ok!(input.parse::<u32>());
        println!("{number}");
    }
}
