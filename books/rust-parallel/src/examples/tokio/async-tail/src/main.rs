use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::PathBuf,
    time::Duration,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let filename = get_filename();
    tail_file(filename).await;
}

async fn tail_file(filename: PathBuf) {
    // Open file and seek to end
    let mut file = File::open(&filename).expect("Failed to open file");
    file.seek(SeekFrom::End(0)).expect("Failed to seek");

    loop {
        sleep(Duration::from_millis(500)).await;
        let mut reader = BufReader::new(&mut file);
        let mut buf = String::new();
        while reader.read_line(&mut buf).expect("Failed to read line") > 0 {
            print!("{}", buf);
            buf.clear();
        }
        reader.stream_position().expect("Failed to get position");
    }
}

fn get_filename() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    PathBuf::from(&args[1])
}
