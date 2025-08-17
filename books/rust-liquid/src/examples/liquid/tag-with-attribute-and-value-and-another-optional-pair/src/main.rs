mod latest_tag;

fn main() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(latest_tag::LatestTag)
        .build()
        .unwrap()
        .parse(r#"{% latest limit=5 %}"#)
        .unwrap();

    let items = &[latest_tag::Item::new("one", 1, "web")];

    let globals = liquid::object!({"items": items});

    let output = template.render(&globals).unwrap();
    assert_eq!(output, r#"<li>one (web)</li>"#);
}
