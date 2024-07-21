use regex::Regex;
use std::sync::Arc;
use tokio::join;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();

    let url = "https://rust.code-maven.com/sitemap.xml";
    let resp = reqwest::get(url).await?;
    //println!("{:#?}", resp);
    //println!("{:#?}", resp.status());
    let text = resp.text().await?;
    //println!("{}", text);
    // <loc>https://rust.code-maven.com/</loc>

    let re = Regex::new(r"<loc>([^<]+)</loc>").unwrap();

    let mut tasks: Vec<JoinHandle<Result<(), ()>>> = vec![];

    let mut count = 0;
    for capture in re.captures_iter(&text) {
        //println!("Full match: '{}'", &capture[0]);
        //println!("URL match: '{}'", &capture[1]);
        let path = Arc::new(capture[1].to_owned());
        {
            let path = path.clone();
            tasks.push(tokio::spawn(async move {
                match reqwest::get(&*path).await {
                    Ok(resp) => match resp.text().await {
                        Ok(text) => {
                            println!("RESPONSE: {} bytes from {}", text.len(), path);
                        }
                        Err(_) => println!("ERROR reading {}", path),
                    },
                    Err(_) => println!("ERROR downloading {}", path),
                }
                Ok(())
            }));
        }
        println!("{path}");

        count += 1;
        if count > 10 {
            break;
        }
    }

    println!("Started {} tasks. Waiting...", tasks.len());
    for task in tasks {
        let r = join!(task);
    }
    //join_all(tasks).await;

    println!("Elapsed time: {:?}", start.elapsed());
    Ok(())
}
