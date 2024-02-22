//use regex::RegexBuilder;
use std::collections::HashMap;

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
    let res = parse_curly(r#"{%}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{%%}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{% %}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{%     %}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{%x%}"#);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("x"),
            fields: HashMap::new()
        })
    );

    let res = parse_curly(r#"{%  y   %}"#);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("y"),
            fields: HashMap::new()
        })
    );

    // TODO should this return an error?
    let res = parse_curly(r#"{%  "   %}"#);
    assert!(res.is_none());
    let res = parse_curly(r#"{%  %   %}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{%  qqrq    %}"#);
    println!("{:?}\n", res);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("qqrq"),
            ..Curly::default()
        })
    );

    let res = parse_curly(r#"{%  qq"rq    %}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{%  include    field%}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{%  include    field  %}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{%  include    field=%}"#);
    assert!(res.is_none());

    let res = parse_curly(r#"{%  include    field=  %}"#);
    assert!(res.is_none());

    // let res = parse_curly(r#"{%  include   file="example/code.rs" %}"#);
    // println!("{:?}\n", res);
    // assert_eq!(
    //     res,
    //     Some(Curly {
    //         name: String::from("include"),
    //         fields: HashMap::from([(String::from("file"), String::from("example/code.rs"))])
    //     })
    // );

    // let res = parse_curly(r#"{%  youtube id="movie"  title="Title" answer=42 %}"#);
    // println!("{:?}\n", res);
    // assert_eq!(
    //     res,
    //     Some(Curly {
    //         name: String::from("youtube"),
    //         fields: HashMap::from([
    //             (String::from("id"), String::from("movie")),
    //             (String::from("title"), String::from("Title"))
    //         ])
    //     })
    // );

    // TODO
    // let res = parse_curly(r#"{%  youtube title="Title \"quoted\" done" %}"#);
    // println!("{:?}\n", res);
    // assert_eq!(
    //     res,
    //     Some(Curly {
    //         name: String::from("youtube"),
    //         fields: HashMap::from([(String::from("title"), String::from("Title \"quoted\" done"))])
    //     })
    // );

    // TODO
    // let res = parse_curly(r#"{%  youtube title="Title with escaped backslash \\" %}"#);
    // println!("{:?}\n", res);
    // assert_eq!(
    //     res,
    //     Some(Curly {
    //         name: String::from("youtube"),
    //         fields: HashMap::from([(String::from("title"), String::from("Title escaped backslas \\"))])
    //     })
    // );
}

fn parse_curly(text: &str) -> Option<Curly> {
    //let mut name = String::new();
    if !text.starts_with("{%") {
        return None;
    }
    if !text.ends_with("%}") {
        return None;
    }
    if text.len() < 5 {
        return None;
    }

    println!("text: '{text}'");

    let chars = text.chars().collect::<Vec<char>>();
    let chars = &chars[2..chars.len() - 2];
    //println!("{:?} {}", chars, chars.len());

    let mut ix = 0;
    let start_name = loop {
        if ix >= chars.len() {
            break None;
        }
        if chars[ix] != ' ' {
            if chars[ix].is_ascii_lowercase() {
                break Some(ix);
            } else {
                break None;
            }
        }
        ix += 1;
    }?;
    //println!("start_name: {start_name:?}");

    let end_name = loop {
        if ix >= chars.len() {
            break Some(ix - 1);
        }
        if chars[ix] == ' ' {
            break Some(ix - 1);
        }
        if ! chars[ix].is_ascii_lowercase() {
            break None;
        }
        ix += 1;
    }?;
    //println!("end_name: {start_name:?}");
    let mut crl = Curly {
        name: text[2 + start_name..3 + end_name].to_owned(),
        ..Curly::default()
    };

    loop {
        let start_field = loop {
            if ix < chars.len() {
                println!("start_key char: {} at {}", chars[ix], ix);
            }

            if ix >= chars.len() {
                return Some(crl);
            }
            if chars[ix] != ' ' {
                if chars[ix].is_ascii_lowercase() {
                    break Some(ix);
                } else {
                    println!("Invalid character '{}' at {} while looking for start_key", chars[ix], ix);
                    break None; // error?
                }
            }
    
            ix += 1;
        }?;

        let end_field = loop {
            if ix < chars.len() {
                println!("end_key: char: {} at {}", chars[ix], ix);
            }

            if ix >= chars.len() {
                // We started a key but have not ended
                break None;
            }
            if chars[ix] == '=' {
                break Some(ix - 1);
            }
            if ! chars[ix].is_ascii_lowercase() {
                break None;
            }
            ix += 1;
        }?;
        
        let field = text[2 + start_field..3 + end_field].to_owned();
        println!("field {}-{} '{}'", start_field, end_field, field);
        crl.fields.insert(field.to_owned(), "hi".to_owned());
        ix += 1;
        if ix >= chars.len() {
            break None;
        }

        let quote = chars[ix] == '"';
        if quote {
            ix += 1;
        }
        if ix >= chars.len() {
            break None;
        }

        let start_value = loop {
            break None;
        }?;

        if ix >= chars.len() {
            break Some(crl);
        }
    }

    // for (ix, ch) in text.chars().enumerate() {
    //     //println!("{ix}  {ch}");
    //          start_name = ix;
    //     }
    //     // //println!("{ch}");
    //     // name = format!("{name}{ch}")
    // }
    //    println!("{name}");

}

