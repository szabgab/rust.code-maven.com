use regex::RegexBuilder;
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

    let res = parse_curly(r#"{%  qqrq    %}"#);
    println!("{:?}\n", res);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("qqrq"),
            ..Curly::default()
        })
    );

    //println!("{:?}\n", res);

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
    println!("{:?} {}", chars, chars.len());

    let mut ix = 0;
    let start_name = loop {
        if ix >= chars.len() {
            break None;
        }
        if chars[ix] != ' ' {
            break Some(ix);
        }
        ix += 1;
    }?;
    println!("start_name: {start_name:?}");

    let end_name = loop {
        if ix >= chars.len() {
            break Some(ix - 1);
        }
        if chars[ix] == ' ' {
            break Some(ix - 1);
        }
        ix += 1;
    }?;
    println!("end_name: {start_name:?}");
    let mut res = Curly {
        name: text[2 + start_name..3 + end_name].to_owned(),
        ..Curly::default()
    };

    // for (ix, ch) in text.chars().enumerate() {
    //     //println!("{ix}  {ch}");
    //          start_name = ix;
    //     }
    //     // //println!("{ch}");
    //     // name = format!("{name}{ch}")
    // }
    //    println!("{name}");

    Some(res)
}

// fn parse_curly(text: &str) -> Option<Curly> {
//     let base_pattern = r#"
//         \{%
//         \s+
//         ([a-z]+)
//         \s+
//         (.*)
//         %\}
//         "#;
//     // ((([a-z]+)="([^"]+)"\s+)*)
//     let base_re = RegexBuilder::new(base_pattern)
//         .ignore_whitespace(true)
//         .build()
//         .unwrap();

//     let pair_pattern = r#"
//     ([a-z]+)=(|"([^"]+)")
//     \s+
//     (.*)
//     "#;

//     let pair_re = RegexBuilder::new(pair_pattern)
//         .ignore_whitespace(true)
//         .build()
//         .unwrap();

//     let mut locs = base_re.capture_locations();
//     match base_re.captures_read(&mut locs, text) {
//         None => None,
//         Some(_cap) => {
//             let loc = locs.get(1).unwrap();
//             let name = &text[loc.0..loc.1];
//             println!("name: {name}");

//             let loc = locs.get(2).unwrap();
//             let mut text = &text[loc.0..loc.1];
//             println!("text {text}");

//             let mut crl = Curly {
//                 name: name.to_string(),
//                 ..Curly::default()
//             };

//             loop {
//                 if text.is_empty() {
//                     break;
//                 }

//                 let mut locs = pair_re.capture_locations();
//                 println!("TEXT: {text}");
//                 match pair_re.captures_read(&mut locs, text) {
//                     None => break,
//                     Some(_cap) => {
//                         let loc = locs.get(1).unwrap();
//                         let field = &text[loc.0..loc.1];
//                         println!("field: '{field}'");

//                         let loc = locs.get(2).unwrap();
//                         let value = &text[loc.0..loc.1];
//                         println!("value: '{value}'");
//                         crl.fields.insert(field.to_owned(), value.to_owned());

//                         match locs.get(4) {
//                             None => {
//                                 println!("break");
//                                 break;
//                             }
//                             Some(loc) => {
//                                 text = &text[loc.0..loc.1];
//                                 println!("new text: {text}");
//                             }
//                         }
//                     }
//                 }
//             }

//             Some(crl)
//         }
//     }
// }
