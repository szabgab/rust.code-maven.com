#![allow(unused_variables)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    let filename = &args
        .get(1)
        .ok_or("Missing file")?;
    let number = args
        .get(2)
        .ok_or("Param 2 was missing")?
        .parse::<u8>()?;
    let num = number
        .checked_add(255)
        .ok_or("overflow")?;


    let content = std::fs::read_to_string(filename)?;

    Ok(())
}
