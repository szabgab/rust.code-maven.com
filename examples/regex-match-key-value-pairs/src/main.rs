use regex::Regex;

fn main() {
    let re = Regex::new(r#"(\w+)="([^"]+)""#).unwrap();

    for text in vec![r#"name="foo""#, "anwser=42"] {
        println!("{text}");
        match re.captures(text) {
            None => {}
            Some(cap) => {
                println!("'{}'", &cap[1]);
                println!("'{}'", &cap[2]);
            }
        }
        println!("---");
    }

    println!("====");

    let re = Regex::new(r#"(\w+)=(?:"([^"]+)"|(\d+))"#).unwrap();

    for text in vec![r#"name="foo""#, "anwser=42"] {
        println!("{text}");
        match re.captures(text) {
            None => {}
            Some(cap) => {
                let key = &cap[1];
                let value = if cap.get(2).is_some() {
                    &cap[2]
                } else {
                    &cap[3]
                };

                println!("'{}'", key);
                println!("'{}'", value);
            }
        }
        println!("---");
    }

    println!("====");

    // title="hello \"foo\" bar"
    // title="hello \\"foo\" bar"
    // title="hello\\"
    let text = r#"  name="foo"  anwser=42  "#;
    println!("{text}");
    for cap in re.captures_iter(text) {
        let key = &cap[1];
        let value = if cap.get(2).is_some() {
            &cap[2]
        } else {
            &cap[3]
        };
        println!("'{}'", key);
        println!("'{}'", value);
        println!("----");
    }

    // for (_, [key, value]) in re.captures_iter(text).map(|cap| {
    //     let key = &cap[1];
    //     let value = if cap.get(2).is_some() {
    //         &cap[2]
    //     } else {
    //         &cap[3]
    //     };
    //     (key, value)
    // }) {
    //     println!("'{}'", key);
    //     println!("'{}'", value);
    // }
}
