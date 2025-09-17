use reqwest::Error;
use scraper::{Html, Selector};

fn get_url() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <urls_file>", args[0]);
        std::process::exit(1);
    }
    let url = &args[1];
    url.to_string()
}

async fn download_page(url: String) -> Result<(String, String), Error> {
    let response = reqwest::get(&url).await?;
    let content = response.text().await?;
    Ok((url, content))
}

async fn crawl(url: &str, limit: u32) {
    let mut tasks = tokio::task::JoinSet::new();
    println!("Downloading: {url}");
    let mut count = 1;
    tasks.spawn({
        let url = url.to_string();
        async move { download_page(url).await }
    });

    println!("Started tasks. Waiting...");
    while !tasks.is_empty() {
        let result = tasks.join_next().await;
        match result {
            Some(Ok(content)) => {
                match content {
                    Ok((url, html)) => {
                        println!("Downloaded {} bytes from {url}", html.len());
                        parse_links(&html).iter().for_each(|link| {
                            if link.starts_with("http://") {
                                //println!("Found unsecure: {link}");
                            }

                            if link.starts_with("https://") {
                                //println!("Found secure link: {link}");
                                if count < limit {
                                    count += 1;
                                    tasks.spawn({
                                        let link = link.to_string();
                                        async move { download_page(link).await }
                                    });
                                }
                            }
                            // internal links
                        });
                    }
                    Err(err) => println!("Error downloading {url}: {err}"),
                }
            }
            Some(Err(err)) => {
                println!("Task failed: {err}");
            }
            None => break,
        }
    }
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();

    let url = get_url();
    println!("Staring with: {url}");
    crawl(&url, 5).await;

    let elapsed = start.elapsed();
    println!("Elapsed time: {elapsed:.2?}");
}

fn parse_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap();

    document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .map(|href| href.to_string())
        .collect()
}
