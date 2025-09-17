use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::Error;

fn get_urls_file() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <urls_file>", args[0]);
        std::process::exit(1);
    }
    let urls_file = &args[1];
    urls_file.to_string()
}

fn read_urls_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?.trim().to_string();
            if line.is_empty() || line.starts_with('#') {
                None
            } else {
                Some(line)
            }
        })
        .collect()
}

async fn download_page(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

async fn downlod_many_urls(urls: &[String]) {
    let mut tasks = tokio::task::JoinSet::new();
    for url in urls {
        println!("Downloading: {url}");
        tasks.spawn({
            let url = url.clone();
            async move {
                match download_page(&url).await {
                    Ok(content) => {
                        println!("Downloaded {} bytes from {url}", content.len());
                    }
                    Err(err) => println!("Error downloading {url}: {err}"),
                }
            }
        });
    }

    println!("Started {} tasks. Waiting...", tasks.len());
    tasks.join_all().await;
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();

    let urls_file = get_urls_file();
    let urls = read_urls_from_file(&urls_file);
    println!("URLs: {urls:?}");

    downlod_many_urls(&urls).await;

    let elapsed = start.elapsed();
    println!("Elapsed time: {elapsed:.2?}");
}
