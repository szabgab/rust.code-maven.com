fn main() {
    let res = concatenate("cat.txt", "dog.txt");
    println!("{:?} {:?}", res.0, res.1);
}

fn concatenate(file1: &str, file2: &str) -> (String, String) {
    let data1 = std::fs::read_to_string(file1).unwrap();
    let data1 = data1.trim_end();
    let data2 = std::fs::read_to_string(file2).unwrap();
    let data2 = data2.trim_end();
    (format!("{data1}{data2}"), format!("{data2}{data1}"))
}
