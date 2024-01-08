fn main() {
    let colors = vec!["blue", "red", "green", "yellow"]
        .into_iter()
        .map(|str| str.to_string())
        .collect::<Vec<String>>();

    let color = infinite_menu(&colors);
    println!("The selected color is {}", color);
}

// Keep asking till we get a valid answer
fn infinite_menu(values: &Vec<String>) -> String {
    loop {
        for (ix, value) in values.iter().enumerate() {
            println!("{}) {}", ix + 1, value);
        }
        let mut response = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Faild to get input");

        match response.trim_end().parse::<usize>() {
            Ok(idx) => {
                if 1 <= idx && idx <= values.len() {
                    return values[idx - 1].clone();
                }
            }
            Err(_err) => {}
        }
        println!("Input must be a number between 1-{}", values.len());
    }
}
