use regex::{Captures, Regex};

fn main() {
    // replace every sequnce of letters by x (but not the digits)
    let text = String::from("abc123def");
    let re = Regex::new(r"[A-Za-z]+").unwrap();
    let result = re.replace_all(&text, "x");
    println!("{result}"); // x123x

    // replace every 3 characters by a y
    let text = String::from("abc123def");
    let re = Regex::new(r"...").unwrap();
    let result = re.replace_all(&text, "y");
    println!("{result}"); // yyy

    // Swap every two charcters
    let text = String::from("abc123def");
    let re = Regex::new(r"(.)(.)").unwrap();
    let result = re.replace_all(&text, r#"$2$1"#);
    println!("{result}"); // ba1c32edf

    // duplicate every digit
    let text = String::from("abc123def");
    let re = Regex::new(r"(\d)").unwrap();
    let result = re.replace_all(&text, r#"$1$1"#);
    println!("{result}"); // abc112233def

    // double every digit
    // Replace using a function to generated the replacement
    let text = String::from("abc123def");
    let re = Regex::new(r"(\d)").unwrap();
    let result = re.replace_all(&text, double);
    println!("{result}"); // abc246def

    // double every digit
    // The same inlined
    let text = String::from("abc123def");
    let re = Regex::new(r"(\d)").unwrap();
    let result = re.replace_all(&text, |caps: &Captures| {
        format!("{}", 2 * caps[1].parse::<u8>().unwrap())
    });
    println!("{result}"); // abc246def
}

fn double(caps: &Captures) -> String {
    //String::new()
    //format!("{}", 2 * caps.get(1).unwrap().as_str().parse::<u8>().unwrap())
    format!("{}", 2 * caps[1].parse::<u8>().unwrap())
}
