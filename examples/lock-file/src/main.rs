extern crate file_lock;

use std::io::Seek;
use std::io::SeekFrom;
use std::path::PathBuf;
use std::error::Error;
use std::fs::read_to_string;
use std::env;
use std::process::exit;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::thread;
use file_lock::{FileLock, FileOptions};


fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} COUNT  THREADS", args[0]);
        exit(1);
    }

    let count = args[1].parse::<u32>()?;
    let threads = args[2].parse::<u32>()?;
    

    let filename = PathBuf::from("counter.txt");
    start_file(&filename)?;


    let mut handles = vec![];
    for _ in 1..=threads {
        let fname = filename.clone();

        handles.push(thread::spawn(move || {
            //counter(count, fname).unwrap();
            counter_with_lock(count, fname).unwrap();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total from the file: {}", read_to_string(&filename)?);

    Ok(())
}

fn start_file(filename: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    write!(&mut file, "0")?;

    Ok(())
}


// fn counter(count: u32, filename: PathBuf)  -> Result<(), Box<dyn Error>> {
//     println!("Starting to count till {count}");
//     for _ in 0..count {       
//         let mut content = String::new();
        
//         let mut file = File::open(&filename)?;    
//         file.read_to_string(&mut content)?;
//         println!("c: {content:?}");

//         let value = content.parse::<u32>()?;

//         let mut file = File::create(&filename)?;
//         write!(&mut file, "{}", value + 1)?;
    
//     }
//     Ok(())
// }


fn counter_with_lock(count: u32, filename: PathBuf)  -> Result<(), Box<dyn Error>> {
    println!("Starting to count till {count}");
    for _ in 0..count {       
        let mut buffer = [0; 100];

        let is_blocking = true;
        let options = FileOptions::new().read(true).write(true);

        let mut filelock = FileLock::lock(&filename, is_blocking, options)?;
        let res = filelock.file.read(&mut buffer)?;

        //println!("c: {:?} size: {res}", buffer);
        let content = String::from_utf8(buffer[0..res].to_vec())?;
        println!("c: {content:?}");

        let value = content.parse::<u32>()?;
        filelock.file.seek(SeekFrom::Start(0))?;

        filelock.file.write_all(format!("{}", value + 1).as_bytes())?;
    }
    Ok(())
}

