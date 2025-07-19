fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("{}", divide(filename));
}


fn divide(filename: &str) -> Result<i32, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let content = content.strip_suffix('\n').unwrap();
    let (dividend, divisor) = content.split_once(',').unwrap();
    let dividend = dividend.parse::<i32>().unwrap();
    let divisor = divisor.parse::<i32>().unwrap();
    dividend / divisor
}


// fn divide(filename: &str) -> i32 {
//     let content = std::fs::read_to_string(filename).unwrap();
//     let content = content.strip_suffix('\n').unwrap();
//     let (dividend, divisor) = content.split_once(',').unwrap();
//     let dividend = dividend.parse::<i32>().unwrap();
//     let divisor = divisor.parse::<i32>().unwrap();
//     dividend / divisor
// }