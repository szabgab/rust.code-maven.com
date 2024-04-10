// fn main() -> Result<(), String> {
//     Ok(())
// }

// fn main() -> Result<(), String> {
//     Err(String::from("A problem"))
// }


// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     Err(Box::<dyn std::error::Error>::from("Problem"))
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    let filename = args.get(1).ok_or("Missing argument")?;
    let text  = std::fs::read_to_string(filename)?;
    println!("{}", text);

    Ok(())
}
