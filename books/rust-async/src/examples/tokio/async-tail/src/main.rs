use std::{env, path::PathBuf, time::Duration};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let filename = get_filename();
    tail_file(filename).await;
}

async fn tail_file(filename: PathBuf) {
    let mut file = File::open(&filename).await.expect("Failed to open file");
    file.seek(SeekFrom::End(0)).await.expect("Failed to seek");

    loop {
        //sleep(Duration::from_millis(500)).await;
        let mut reader = BufReader::new(&mut file);
        let mut buf = String::new();
        while reader
            .read_line(&mut buf)
            .await
            .expect("Failed to read line")
            > 0
        {
            print!("{}", buf);
            buf.clear();
        }
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
