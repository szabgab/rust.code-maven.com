use mime_guess::mime;

fn main() {
    let guess = mime_guess::from_path("some_file.gif");
    assert_eq!(guess.first(), Some(mime::IMAGE_GIF));
    println!("{}", guess.first().unwrap()); // image/gif

    println!("{}", mime_guess::from_path("some.gif").first().unwrap());  // image/gif
    println!("{}", mime_guess::from_path("some.js").first().unwrap());  // application/javascript
    println!("{}", mime_guess::from_path("some.json").first().unwrap()); // application/json
    println!("{}", mime_guess::from_path("some.yaml").first().unwrap()); // text/x-yaml
    println!("{}", mime_guess::from_path("some.yml").first().unwrap()); // text/x-yaml
    println!("{}", mime_guess::from_path("some.html").first().unwrap()); // text/html
    println!("{}", mime_guess::from_path("some.py").first().unwrap()); // text/plain
    println!("{}", mime_guess::from_path("some.rs").first().unwrap()); // text/x-rust

    assert!(mime_guess::from_path("some.qqrq").first().is_none());
    assert!(mime_guess::from_path("some").first().is_none());
}
