use std::{env, fs::File, io::{BufRead, BufReader, Seek, SeekFrom}, path::PathBuf};
use tokio::sync::mpsc;
use notify::{Watcher, RecursiveMode, EventKind, RecommendedWatcher};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = PathBuf::from(&args[1]);

    let (tx, mut rx) = mpsc::channel(1);
    let tx2 = tx.clone();

    // Set up file watcher
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Modify(_)) {
                    let _ = tx2.try_send(());
                }
            }
        },
        notify::Config::default()
    ).expect("Failed to create watcher");
    watcher.watch(&filename, RecursiveMode::NonRecursive).expect("Failed to watch file");

    // Open file and seek to end
    let mut file = File::open(&filename).expect("Failed to open file");
    let mut pos = file.seek(SeekFrom::End(0)).expect("Failed to seek");

    loop {
        // Wait for a file change event
        rx.recv().await;
        // Re-open file and seek to last position
        let mut file = File::open(&filename).expect("Failed to open file");
        file.seek(SeekFrom::Start(pos)).expect("Failed to seek");
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        while reader.read_line(&mut buf).expect("Failed to read line") > 0 {
            print!("{}", buf);
            buf.clear();
        }
        pos = reader.stream_position().expect("Failed to get position");
    }
}
