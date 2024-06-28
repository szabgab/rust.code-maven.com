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

use advisory_lock::{AdvisoryFileLock, FileLockMode};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        eprintln!("Usage: {} COUNT  THREADS  [lock|separate|once]", args[0]);
        exit(1);
    }

    let count = args[1].parse::<u32>()?;
    let threads = args[2].parse::<u32>()?;
    let mode = args[3];
    

    let filename = PathBuf::from("counter.txt");
    start_file(&filename)?;

    let mut handles = vec![];
    for _ in 1..=threads {
        let fname = filename.clone();

        handles.push(thread::spawn(move || {
            match &mode {
                "lock" => counter_with_lock(count, fname).unwrap(),
                "separate" => counter_separate(count, fname).unwrap(),
                "once" => counter_once(count, fname),
                _ => panic!("Invalid"),
            }
            
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


fn counter_with_lock(count: u32, filename: PathBuf)  -> Result<(), Box<dyn Error>> {
    println!("Starting to count till {count}");
    for _ in 0..count {       
        let mut buffer = [0; 100];

        let mut file =  File::options().read(true).write(true).create(true).open(&filename)?;
        file.lock(FileLockMode::Exclusive)?;
       
        let res = file.read(&mut buffer)?;

        let content = String::from_utf8(buffer[0..res].to_vec())?;
        println!("c: {content:?}");

        let mut value = content.parse::<u32>()?;
        value += 1;

        file.seek(SeekFrom::Start(0))?;
        // no need to truncate as the length of the number is just growing
        file.write_all(format!("{}", value).as_bytes())?;
    }

    Ok(())
}


fn counter_separate(count: u32, filename: PathBuf)  -> Result<(), Box<dyn Error>> {
    println!("Starting to count till {count}");
    for _ in 0..count {       
        let mut content = String::new();
        
        let mut file = File::open(&filename)?;    
        file.read_to_string(&mut content)?;
        println!("c: {content:?}");

        let mut value = content.parse::<u32>()?;
        value += 1;

        let mut file = File::create(&filename)?;
        write!(&mut file, "{}", value)?;
    
    }
    Ok(())
}


fn counter_once(count: u32, filename: PathBuf)  -> Result<(), Box<dyn Error>> {
    println!("Starting to count till {count}");
    for _ in 0..count {       
        let mut content = String::new();
       
        let mut file =  File::options().read(true).write(true).create(true).open(&filename)?;
        file.read_to_string(&mut content)?;
        println!("c: {content:?}");

        let mut value = content.parse::<u32>()?;
        value += 1;

        file.seek(SeekFrom::Start(0))?;
        // no need to truncate as the length of the number is just growing
        write!(&mut file, "{}", value ?;
    
    }
    Ok(())
}
