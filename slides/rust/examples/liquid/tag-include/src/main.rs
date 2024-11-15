mod include_tag;

fn main() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(include_tag::IncludeTag)
        .build()
        .unwrap()
        .parse(r#"{% include file="files/hello.txt" %}"#)
        .unwrap();

    let globals = liquid::object!({});

    let output = template.render(&globals).unwrap();
    assert_eq!(output, "This is the hello file.\n");
}
