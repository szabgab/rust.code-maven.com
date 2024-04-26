use std::fs::File;
use std::io::Write;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

    let filename = "data.txt";
    let mut file = File::create(filename)?;
    writeln!(&mut file, "Hello World!")?;

    Ok(())
}
