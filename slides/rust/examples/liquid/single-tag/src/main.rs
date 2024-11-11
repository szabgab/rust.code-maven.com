mod single_tag;

fn main() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(single_tag::SingleTag)
        .build()
        .unwrap()
        .parse("Liquid: {% single %}")
        .unwrap();

    let globals = liquid::object!({});

    let output = template.render(&globals).unwrap();
    assert_eq!(
        output,
        "Liquid: Single replaced by this string.".to_string()
    );
}
