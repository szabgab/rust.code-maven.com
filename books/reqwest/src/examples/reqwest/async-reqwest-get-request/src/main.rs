use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Response {
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
    origin: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = get_url("get");

    let resp = reqwest::get(url).await?.json::<Response>().await?;
    println!("{:#?}", resp);
    Ok(())
}

fn get_url(path: &str) -> String {
    let host = std::env::args().nth(1).unwrap_or("httpbin.org".into());
    let url = if host == "localhost" {
        format!("http://localhost/{path}")
    } else {
        format!("https://{host}/{path}")
    };

    url
}
