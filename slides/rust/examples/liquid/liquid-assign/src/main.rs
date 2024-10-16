use liquid::partials::{EagerCompiler, InMemorySource};
use std::fs::read_to_string;

pub type Partials = EagerCompiler<InMemorySource>;

fn main() {
    let mut partials = Partials::empty();
    let filename = "templates/incl/header.txt";
    let template = read_to_string(filename).unwrap();
    partials.add(filename, template);

    let template = liquid::ParserBuilder::with_stdlib()
        .partials(partials)
        .build()
        .unwrap()
        .parse_file("templates/page.txt")
        .unwrap();

    let globals = liquid::object!({
        "title": "Liquid",
        "name": "Foo Bar",
        "value": "some value",
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);

    let expected = std::fs::read_to_string("out.txt").unwrap();
    assert_eq!(output.trim_end(), expected.trim_end())
}
