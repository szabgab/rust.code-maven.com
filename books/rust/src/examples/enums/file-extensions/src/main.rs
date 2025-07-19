#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Language {
    Perl,
    Python,
    Rust,
}

impl Language {
    fn from_ext(ext: &str) -> Self {
        match ext {
            "pl" => Language::Perl,
            "py" => Language::Python,
            _ => panic!("No such language"),
        }
    }

    fn option_from_ext(ext: &str) -> Option<Self> {
        match ext {
            "pl" => Some(Language::Perl),
            "py" => Some(Language::Python),
            _ => panic!("No such language"),
        }
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_func() {
        let lang = Language::from_ext("pl");
        assert_eq!(lang, Language::Perl);
    }
}

// TODO how to handle when the user callse from_ext with an unknonw extension?
// Have a variant Unknown and set that?
// panic! ?
// Should the from_ext return an Option  and in this should it be None?
// Should the from_ext return a Result  and in this should it be Error?
