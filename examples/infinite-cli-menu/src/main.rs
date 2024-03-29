fn main() {
    let path = "colors.txt";

    let colors = match std::fs::read_to_string(path) {
        Ok(val) => {
            val.split_terminator('\n').map(|row| row.to_owned()).collect::<Vec<String>>()
        },
        Err(err) => {
            eprintln!("Could no read '{}' Error: {}", path, err);
            std::process::exit(1);
        }
    };

    let color = infinite_menu(colors);
    println!("The selected color is {}", color);
}

fn infinite_menu(values: Vec<String>) -> String {
    loop {
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
                    return values[idx - 1].clone();
                }
            }
            Err(err) => eprintln!("{}", err),
        }
        println!("Input must be a number between 1-{}", values.len());
    }
}
