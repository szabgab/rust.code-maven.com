fn main() {
    let text = get_text();
    let counter = count_digits(&text);
    display(&counter);
}

fn count_digits(text: &str) -> [i8; 10] {
    let mut counter: [i8; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for ch in text.chars() {
        if ch == ' ' {
            continue;
        }
        if !"0123456789".contains(ch) {
            continue;
        }

        let ix = ch as usize - '0' as usize;
        //println!("{ch}");
        //println!("{ix}");
        counter[ix] += 1;
    }

    counter
}

fn display(counter: &[i8]) {
    for (ix, count) in counter.iter().enumerate() {
        println!("{ix}: {}", count);
    }
}

fn get_text() -> String {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1)
    }

    std::fs::read_to_string(&args[1]).unwrap()
}
