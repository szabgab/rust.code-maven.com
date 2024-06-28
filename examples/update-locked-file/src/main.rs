extern crate file_lock;

use std::io::Seek;
use std::io::SeekFrom;
use std::env;
use std::process::exit;
use std::error::Error;
use std::io::Read;
use std::io::Write;

use file_lock::{FileLock, FileOptions};


fn main() -> Result<(), Box<dyn Error>>  {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} TEXT", args[0]);
        exit(1);
    }

    let filename = "message.txt";


    let mut buffer = [0; 1000];
    let is_blocking = true;
    let options = FileOptions::new().read(true).write(true).create(true);

    let mut filelock = FileLock::lock(&filename, is_blocking, options)?;
    let res = filelock.file.read(&mut buffer)?;

    let content = String::from_utf8(buffer[0..res].to_vec())?;
    println!("old: {content:?}");

    filelock.file.seek(SeekFrom::Start(0))?;
    filelock.file.set_len(0)?; // truncate

    filelock.file.write_all(args[1].as_bytes())?;

    Ok(())
}
