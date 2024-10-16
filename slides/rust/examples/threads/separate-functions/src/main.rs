fn main() {
    // let (threads, filename) = get_args();

    println!("Hello, world!");
}

// fn get_args() -> (u32, String) {
//     let args = std::env::args().collect::<Vec<String>>();
//     if args.len() != 3 {
//         eprintln!("Usage: {} THREADS, FILE", args[0]);
//         std::process::exit(1);
//     }

//     (args[1].parse().unwrap(), args[2].to_owned())
// }

// The idea here is to create CPU intensive functions to be able to show the impact ot threading.
// It is not to create the most optimal functions

// return the first character that repeates itself
fn find_double_characters(text: &str, mut nth: u32) -> Option<char> {
    if text.len() < 2 {
        return None;
    }
    if nth < 1 {
        return None;
    }

    let mut resp = None;
    for chr in text.chars() {
        if resp.is_none() {
            resp = Some(chr);
            continue;
        }
        if Some(chr) == resp {
            if nth == 1 {
                return resp;
            } else {
                nth -= 1;
            }
        }
        resp = Some(chr);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_double_characters() {
        assert_eq!(find_double_characters("", 1), None);
        assert_eq!(find_double_characters("aa", 0), None);
        assert_eq!(find_double_characters("a", 1), None);
        assert_eq!(find_double_characters("aa", 1), Some('a'));
        assert_eq!(find_double_characters("aabb", 1), Some('a'));
        assert_eq!(find_double_characters("aabb", 2), Some('b'));
    }
}
