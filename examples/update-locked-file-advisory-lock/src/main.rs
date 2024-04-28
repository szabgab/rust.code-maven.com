use std::io::Seek;
use std::io::SeekFrom;
use std::env;
use std::process::exit;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use advisory_lock::{AdvisoryFileLock, FileLockMode, FileLockError};

fn main() -> Result<(), Box<dyn Error>>  {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} TEXT", args[0]);
        exit(1);
    }

    let filename = "message.txt";


    let mut buffer = [0; 1000];

    let mut file =  File::options().read(true).write(true).create(true).open(filename)?;
    file.lock(FileLockMode::Exclusive)?;

    let res = file.read(&mut buffer)?;

    let content = String::from_utf8(buffer[0..res].to_vec())?;
    println!("old: {content:?}");

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?; // truncate

    file.write_all(args[1].as_bytes())?;

    Ok(())
}
