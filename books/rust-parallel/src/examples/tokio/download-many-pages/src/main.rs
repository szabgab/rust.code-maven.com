
use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::Error;

async fn download_page(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
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


#[tokio::main]
async fn main() {
    let urls = read_urls_from_file("urls.txt");
    println!("URLs: {:?}", urls);

    for url in &urls {
        match download_page(url).await {
            Ok(content) => {
                println!("Downloaded {} bytes from {}", content.len(), url);
            }
            Err(e) => println!("Error downloading {}: {}", url, e),
        }
    }
}
