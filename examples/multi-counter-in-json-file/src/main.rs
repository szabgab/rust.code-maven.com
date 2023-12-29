use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
    let path = std::path::Path::new("counter.json");

    let mut counters: HashMap<String, u32> = if path.exists() {
        let content = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&content).unwrap()
    } else {
        HashMap::new()
    };

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        for (key, value) in counters.iter() {
            println!("{key} {value}");
        }
        return;
    }

    if args.len() == 2 {
        let field = &args[1];
        *counters.entry(field.to_string()).or_insert(0) += 1;
        println!("{}", counters.get(field).unwrap());

        let json_string = serde_json::to_string(&counters).unwrap();
        let mut file = File::create(path).unwrap();
        writeln!(&mut file, "{}", json_string).unwrap();
        return;
    }

    eprintln!("Usage: {}", &args[0]);
    eprintln!("Usage: {} field", &args[0]);
}
