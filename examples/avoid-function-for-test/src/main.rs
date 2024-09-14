use avoid_function_for_test::diff;

fn main() {
    let result = diff("2024.01.01", "2024.01.02");
    println!("{result}");


    let result = diff("2024.01.02", "2024.01.03");
    println!("{result}");
}