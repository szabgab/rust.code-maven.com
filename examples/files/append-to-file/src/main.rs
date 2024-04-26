use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} MESSAGE", args[0]);
        exit(1);
    }

    let message = &args[1];

    let filename = "messages.txt";
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(filename)?;
    writeln!(&mut file, "{message}")?;

    Ok(())
}
