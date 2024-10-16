fn main() {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(
            "
           plain: {{text}}
           upcase: {{text | upcase}}
           downcase: {{text | downcase}}
           capitalize: {{text | capitalize}}
        ",
        )
        .unwrap();

    let text = "this is Some tExt";

    let globals = liquid::object!({
        "text": text,
    });
    let output = template.render(&globals).unwrap();

    println!("{}", output);

    // verify
    let expected = std::fs::read_to_string("out.txt").unwrap();
    assert_eq!(output.trim(), expected.trim());
}
