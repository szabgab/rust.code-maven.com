fn main() {
    for site in [
        String::from("https://rust.code-maven.com"),
        String::from("https://rust.code-maven.com/"),
    ] {
        process(url::Url::parse(&site).unwrap());
    }
}

fn process(site: url::Url) {
    fetch(&site);
    fetch(&site.join("page").unwrap());
}

fn fetch(site: &url::Url) {
    println!("Process '{}'", site);
}
