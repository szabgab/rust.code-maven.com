fn main() {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(
            "
           plain: {{whole}}
           plus 2: {{whole | plus : 2}}
           minus 2 {{whole | minus : 2}}

           plain: {{float}}
           plus 2: {{float | plus : 2}}
           minus 2 {{float | minus : 2}}
        ",
        )
        .unwrap();

    let whole = 42;
    let float = 4.2;

    let globals = liquid::object!({
        "whole": whole,
        "float": float,
    });
    let output = template.render(&globals).unwrap();

    println!("{}", output);

    // verify
    let expected = std::fs::read_to_string("out.txt").unwrap();
    assert_eq!(output.trim(), expected.trim());
}
