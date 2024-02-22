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

#[allow(dead_code)]
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
        if !chars[ix].is_ascii_lowercase() {
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
                println!("start_field char: {} at {}", chars[ix], ix);
            }

            if ix >= chars.len() {
                return Some(crl);
            }
            if chars[ix] != ' ' {
                if chars[ix].is_ascii_lowercase() {
                    break Some(ix);
                } else {
                    eprintln!(
                        "Invalid character '{}' at {} while looking for start_field",
                        chars[ix], ix
                    );
                    break None; // error?
                }
            }

            ix += 1;
        }?;

        let end_field = loop {
            if ix < chars.len() {
                println!("end_field: char: {} at {}", chars[ix], ix);
            }

            if ix >= chars.len() {
                // We started a field but have not ended
                break None;
            }
            if chars[ix] == '=' {
                break Some(ix - 1);
            }
            if !chars[ix].is_ascii_lowercase() {
                break None;
            }
            ix += 1;
        }?;

        let field = text[2 + start_field..3 + end_field].to_owned();
        println!("field {}-{} '{}'", start_field, end_field, field);

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

        let start_value = ix;
        ix += 1;
        if ix >= chars.len() {
            break None;
        }

        let end_value = loop {
            if ix < chars.len() {
                println!("end_value: char: {} at {}", chars[ix], ix);
            }

            if quote {
                //println!("in quoted {} of {}", ix, chars.len());
                if ix >= chars.len() {
                    eprintln!("Quoted value ended before closing quotes");
                    break None;
                }

                if chars[ix] == '"' {
                    ix += 1;
                    if ix >= chars.len() {
                        break None; // require space after the last quoted value
                    }
                    break Some(ix - 2);
                }

                if chars[ix] == '\\' {
                    if chars[ix + 1] == '"' {
                        ix += 2;
                        continue;
                    }
                    if chars[ix + 1] == '\\' {
                        ix += 2;
                        continue;
                    }
                }
            } else {
                if ix >= chars.len() {
                    break None; // require space at the end
                                //break Some(ix - 1); // allow lack of space at the end
                }

                if chars[ix] == ' ' {
                    break Some(ix - 1);
                }

                if !chars[ix].is_ascii_digit() {
                    break None;
                }
            }
            ix += 1;
        }?;

        if start_value == end_value {
            break None;
        }
        let value = text[2 + start_value..3 + end_value].to_owned();
        println!("value {}-{} '{}'", start_value, end_value, value);
        crl.fields.insert(field.to_owned(), value.to_owned());

        if ix >= chars.len() {
            break Some(crl);
        }
    }
}

#[test]
fn test_01() {
    let res = parse_curly(r#"{%}"#);
    assert!(res.is_none());
}

#[test]
fn test_02() {
    let res = parse_curly(r#"{%%}"#);
    assert!(res.is_none());
}

#[test]
fn test_03() {
    let res = parse_curly(r#"{% %}"#);
    assert!(res.is_none());
}

#[test]
fn test_04() {
    let res = parse_curly(r#"{%     %}"#);
    assert!(res.is_none());
}

#[test]
fn test_05() {
    let res = parse_curly(r#"{%x%}"#);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("x"),
            fields: HashMap::new()
        })
    );
}

#[test]
fn test_06() {
    let res = parse_curly(r#"{%  y   %}"#);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("y"),
            fields: HashMap::new()
        })
    );
}

#[test]
fn test_07() {
    // TODO should this return an error?
    let res = parse_curly(r#"{%  "   %}"#);
    assert!(res.is_none());
    let res = parse_curly(r#"{%  %   %}"#);
    assert!(res.is_none());
}

#[test]
fn test_08() {
    let res = parse_curly(r#"{%  qqrq    %}"#);
    println!("{:?}\n", res);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("qqrq"),
            ..Curly::default()
        })
    );
}

#[test]
fn test_09() {
    let res = parse_curly(r#"{%  qq"rq    %}"#);
    assert!(res.is_none());
}

#[test]
fn test_10() {
    let res = parse_curly(r#"{%  include    field%}"#);
    assert!(res.is_none());
}

#[test]
fn test_11() {
    let res = parse_curly(r#"{%  include    field  %}"#);
    assert!(res.is_none());
}

#[test]
fn test_12() {
    let res = parse_curly(r#"{%  include    field=%}"#);
    assert!(res.is_none());
}

#[test]
fn test_13() {
    let res = parse_curly(r#"{%  include    field=  %}"#);
    assert!(res.is_none());
}

#[test]
fn test_14() {
    let res = parse_curly(r#"{%  include   file="example/code.rs" %}"#);
    println!("{:?}\n", res);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("include"),
            fields: HashMap::from([(String::from("file"), String::from("example/code.rs"))])
        })
    );
}

#[test]
fn test_15() {
    let res = parse_curly(r#"{%  youtube id="movie"  title="Title" answer=42 %}"#);
    //println!("{:?}\n", res);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("youtube"),
            fields: HashMap::from([
                (String::from("id"), String::from("movie")),
                (String::from("title"), String::from("Title")),
                (String::from("answer"), String::from("42"))
            ])
        })
    );
}

#[test]
fn test_17() {
    let res = parse_curly(r#"{%  youtube id="movie"  title="Title %}"#);
    //println!("{:?}\n", res);
    assert!(res.is_none());
}

// Let's make sure there is a space between the last value and the closing tag
#[test]
fn test_18() {
    let res = parse_curly(r#"{%  youtube id="movie"  answer=42%}"#);
    //println!("{:?}\n", res);
    assert!(res.is_none());
    // assert_eq!(
    //     res,
    //     Some(Curly {
    //         name: String::from("youtube"),
    //         fields: HashMap::from([
    //             (String::from("id"), String::from("movie")),
    //             (String::from("answer"), String::from("42"))
    //         ])
    //     })
    // );
}

#[test]
fn test_19() {
    let res = parse_curly(r#"{%  youtube id="movie"  title="Title"%}"#);
    println!("{:?}\n", res);
    assert!(res.is_none());
}

#[test]
fn test_90() {
    // TODO
    let res = parse_curly(r#"{%  youtube title="Title \"quoted\" done" %}"#);
    println!("{:?}\n", res);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("youtube"),
            fields: HashMap::from([(
                String::from("title"),
                String::from("Title \\\"quoted\\\" done")
            )]) // TODO why do we have an extra pair of backslashes here?
        })
    );
}

#[test]
fn test_91() {
    // TODO
    let res = parse_curly(r#"{%  youtube title="Title with escaped backslash \\" %}"#);
    println!("{:?}\n", res);
    assert_eq!(
        res,
        Some(Curly {
            name: String::from("youtube"),
            fields: HashMap::from([(
                String::from("title"),
                String::from("Title with escaped backslash \\\\")
            )]) // TODO why do we have an extra pair of backslashes here?
        })
    );
}
