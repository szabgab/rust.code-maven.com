use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = get_url("headers");

    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
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
