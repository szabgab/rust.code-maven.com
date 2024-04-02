use toml::Table;

fn main() {
    let filename = "sample-rustfmt.toml";
    let content = std::fs::read_to_string(filename).unwrap();

    let table = content.parse::<Table>().unwrap();
    //println!("{}", value);
    for row in table.iter() {
        println!("key: {:30} value: {}", row.0, row.1);
    }
}
