macro_rules! prt {
    ($var:expr) => {
        println!(
            "{:2} {:2} {:p} {:15?} '{}'",
            $var.len(),
            $var.capacity(),
            &$var,
            $var.as_ptr(),
            $var
        );
    };
}

fn main() {
    let res = read_file("cat.txt");
    println!("{:?}", res);
    prt!(res);
}

// fn read_file(file: &str) -> &str {
//     let data = std::fs::read_to_string(file).unwrap();
//     let short = data.trim_end();
//     short
//     // cannot return value referencing local variable `data`
//     // returns a value referencing data owned by the current function
// }


fn read_file(file: &str) -> String {
    let data = std::fs::read_to_string(file).unwrap();
    prt!(data);
    let short = data.trim_end();
    //prt!(short);
    short.to_owned() // this to_owned also copies the data
}

