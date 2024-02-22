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
