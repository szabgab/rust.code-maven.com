mod show_tag;

fn main() {
    show_one();
    show_two();
}

fn show_one() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(show_tag::ShowTag)
        .build()
        .unwrap()
        .parse(r#"{% show name %}"#)
        .unwrap();

    let globals = liquid::object!({"name": "Sancho Panza"});

    let output = template.render(&globals).unwrap();
    assert_eq!(output, "Sancho Panza");
}

fn show_two() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(show_tag::ShowTag)
        .build()
        .unwrap()
        .parse(r#"{% show name %} {% show number %}"#)
        .unwrap();

    let globals = liquid::object!({"name": "Sancho Panza", "number": 42});

    let output = template.render(&globals).unwrap();
    assert_eq!(output, "Sancho Panza 42");
}
