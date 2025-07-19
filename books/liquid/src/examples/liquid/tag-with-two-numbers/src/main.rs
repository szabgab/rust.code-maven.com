mod add_tag;

fn main() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(add_tag::AddNumbersTag)
        .build()
        .unwrap()
        .parse("{% add 2 4 %}")
        .unwrap();

    let globals = liquid::object!({});

    let output = template.render(&globals).unwrap();
    assert_eq!(output, "2 + 4 = 6");
}
