#![allow(unused_variables)]

fn main() {
    let res = run();
    match res {
        Ok(_val) => println!("Ok"),
        Err(err) => println!("Error: {err}"),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    read_file("Cargo.toml")?;
    //read_file("a.txt")?;
    convert_number("23")?;
    //convert_number("hello")?;
    increment_number(0)?;
    //increment_number(10)?;

    Ok(())
}

fn read_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    Ok(content)
}


fn convert_number(text: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let number = text.parse::<u8>()?;

    Ok(number)
}

fn increment_number(number: u8) -> Result<u8, Box<dyn std::error::Error>> {   
    let num = number.checked_add(255).ok_or("overflow")?;

    Ok(num)
}
