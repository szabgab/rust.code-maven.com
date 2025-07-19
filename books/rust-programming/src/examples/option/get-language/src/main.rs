#![allow(dead_code)]
fn main() {
    for ext in ["pl", "c"] {
        let lang = get_language(ext);
        println!("{:?}", lang);
        match lang {
            None => (),
            Some(val) => println!("{}", val),
        };

        if let Some(val) = lang {
            println!("if: {}", val);
        } else {
            println!("we had none");
        }
    }
}

fn get_language_empty_string(ext: &str) -> &str {
    if ext == "rs" {
        return "rust";
    }
    if ext == "py" {
        return "python";
    }
    if ext == "pl" {
        return "perl";
    }

    ""
}

fn get_language_empty_string_match(ext: &str) -> &str {
    match ext {
        "rs" => "rust",
        "py" => "python",
        "pl" => "perl",
        _ => "",
    }
}

fn get_language(ext: &str) -> Option<&str> {
    match ext {
        "rs" => Some("rust"),
        "py" => Some("python"),
        "pl" => Some("perl"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_language_empty_string() {
        assert_eq!(get_language_empty_string("rs"), "rust");
        assert_eq!(get_language_empty_string("py"), "python");
        assert_eq!(get_language_empty_string("pl"), "perl");
        assert_eq!(get_language_empty_string("qqrq"), "");
    }

    #[test]
    fn test_get_language_empty_string_match() {
        assert_eq!(get_language_empty_string_match("rs"), "rust");
        assert_eq!(get_language_empty_string_match("py"), "python");
        assert_eq!(get_language_empty_string_match("pl"), "perl");
        assert_eq!(get_language_empty_string_match("qqrq"), "");
    }

    #[test]
    fn test_get_language() {
        assert_eq!(get_language("rs"), Some("rust"));
        assert_eq!(get_language("py"), Some("python"));
        assert_eq!(get_language("pl"), Some("perl"));
        assert_eq!(get_language("qqrq"), None);
    }
}
