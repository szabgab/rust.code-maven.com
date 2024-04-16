fn main() {
    let mut path = std::path::PathBuf::from("hello");
    println!("{:?}", path); // "hello"

    // adds an extension
    path.set_extension("txt");
    println!("{:?}", path); // "hello.txt"

    // replaces an extension
    path.set_extension("html");
    println!("{:?}", path); // "hello.html"

    // If we have a file that has a . in the name, for example
    let mut path = std::path::PathBuf::from("hello.0");
    println!("{:?}", path); // "hello.0"

    // and we would like to add an extension, this will not do what we
    // want as this will replace the 0 by the html
    path.set_extension("html");
    println!("{:?}", path); // "hello.html"

    let path = std::path::PathBuf::from("hello.0");
    println!("{:?}", path); // "hello.0"
    println!("{}", format!("{}.html", path.display())); // hello.0.html

    let path = std::path::PathBuf::from(format!("{}.html", path.display()));
    println!("{:?}", path); // "hello.0.html"
}
