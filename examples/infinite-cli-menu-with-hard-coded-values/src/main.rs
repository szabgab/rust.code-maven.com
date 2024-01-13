fn main() {
    let colors = vec!["blue", "red", "green", "yellow"];

    let color = infinite_menu(colors);
    println!("The selected color is {}", color);
}

fn infinite_menu(values: Vec<&str>) -> &str {
    loop {
        for (ix, value) in values.iter().enumerate() {
            println!("{}) {}", ix + 1, value);
        }

        let mut response = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to get input");

        // match std::io::stdin().read_line(&mut response) {
        //     Ok(_) => {},
        //     Err(err) => {
        //         eprintln!("Error: {}", err);
        //         continue;
        //     }
        // }

        match response.trim_end().parse::<usize>() {
            Ok(idx) => {
                if 1 <= idx && idx <= values.len() {
                    return values[idx - 1];
                }
            }
            Err(_err) => {}
        }
        println!("Input must be a number between 1-{}", values.len());
    }
}
