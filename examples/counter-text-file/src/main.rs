use std::fs::File;
use std::io::Write;

fn main() {
    let path = std::path::Path::new("counter.txt");

    let counter: u32 = if path.exists() {
        std::fs::read_to_string(path)
            .unwrap()
            .trim_end()
            .parse()
            .unwrap()
    } else {
        0
    };

    let counter = counter + 1;

    let mut fh = File::create(path).unwrap();
    writeln!(&mut fh, "{}", counter).unwrap();

    println!("{counter}");
}
