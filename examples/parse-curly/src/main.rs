use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
struct Curly {
    name: String,
    fields: HashMap<String, String>,
}

impl Default for Curly {
    fn default() -> Self {
        Self {
            name: String::new(),
            fields: HashMap::new(),
        }
    }
}


fn main() {
    let res = parse_curly(r#"{%  qqrq    %}"#);
    assert_eq!(res, Some(Curly { name: String::from("qqrq"), ..Curly::default()}));

    let res = parse_curly(r#"{%  include   file="example/code.rs" %}"#);

    println!("{:?}", res);
}


fn parse_curly(text: &str) -> Option<Curly> {
    let re = Regex::new(r#"\{%\s+(?<name>[a-z]+)\s+(([a-z]+)="([^"]+)"\s+)*%\}"#).unwrap();
    match re.captures(text) {
        None => None,
        Some(cap) => {
            //cap.
            Some(Curly {
                name: cap["name"].to_owned(),
                fields: HashMap::new(),
            })
            
        }
    }
}
