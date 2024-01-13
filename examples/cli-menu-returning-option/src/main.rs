fn main() {
    let colors = vec!["blue", "red", "green", "yellow"]
        .into_iter()
        .map(|str| str.to_string())
        .collect::<Vec<String>>();

    let color = match menu(&colors) {
        Some(val) => val,
        None => {
            eprintln!("No valid value was provided");
            std::process::exit(1);
        }
    };
    println!("The selected color is {}", color);
}

fn menu(values: &Vec<String>) -> Option<String> {
        for (ix, value) in values.iter().enumerate() {
            println!("{}) {}", ix + 1, value);
        }
        let mut response = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to get input");

        match response.trim_end().parse::<usize>() {
            Ok(idx) => {
                if 1 <= idx && idx <= values.len() {
                    return Some(values[idx - 1].clone());
                }
            }
            Err(_err) => {}
        }
        None
}
