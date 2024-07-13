use std::collections::HashMap;

fn main() {
    let (threads, files) = get_args();
    if threads == 1 {
        let mut total: HashMap<char, u32> = HashMap::new();
        for file in files {
            let text = std::fs::read_to_string(file).unwrap();
            let data = count_characters(&text);
            println!("{:#?}", &data);
            add(&mut total, &data);
            println!("{:#?}", total);
        }    
    }
}

fn get_args() -> (u32, Vec<String>) {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Usage: {} THREADS, FILEs", args[0]);
        std::process::exit(1);
    }

    (args[1].parse().unwrap(), args[2..].to_owned())
}

fn count_characters(text: &str) -> HashMap<char, u32> {
    let mut counter: HashMap<char, u32> = HashMap::new();
    for ch in text.chars() {
        *counter.entry(ch).or_insert(0) += 1;
    }

    counter
}

fn add(total: &mut HashMap<char, u32>, other : &HashMap<char, u32>) {
    for (key, value) in other.iter() {
        *total.entry(*key).or_insert(0) += value;
    }
}
