use std::env;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};
use tokio::time::{Duration, sleep};

async fn tail_file(path: PathBuf) {
    println!("Tailing file: {}", path.display());
    let mut file = match File::open(&path).await {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open {}: {}", path.display(), e);
            return;
        }
    };
    file.seek(SeekFrom::End(0)).await.expect("Failed to seek");
    loop {
        sleep(Duration::from_millis(50)).await;
        let mut reader = BufReader::new(&mut file);
        let mut buf = String::new();
        while reader
            .read_line(&mut buf)
            .await
            .expect("Failed to read line")
            > 0
        {
            print!("{} {}", path.file_name().unwrap().to_string_lossy(), buf);
            buf.clear();
        }
    }
}

fn get_filenames() -> Vec<PathBuf> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: async-multi-tail <file1> <file2> ...");
        return Vec::new();
    }
    let paths: Vec<PathBuf> = args.into_iter().map(PathBuf::from).collect();

    paths
}

#[tokio::main]
async fn main() {
    let file_paths = get_filenames();
    let mut handles = Vec::new();
    for file_path in file_paths {
        handles.push(tokio::spawn(tail_file(file_path)));
    }
    for handle in handles {
        let _ = handle.await;
    }
}
