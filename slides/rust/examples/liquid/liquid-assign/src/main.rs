use std::fs::read_to_string;
use liquid::partials::{EagerCompiler, InMemorySource};

pub type Partials = EagerCompiler<InMemorySource>;

fn main() {
    let mut partials = Partials::empty();
    let filename ="templates/incl/header.txt";
    let template = read_to_string(filename).unwrap();
    partials.add(filename, template);

    let template = liquid::ParserBuilder::with_stdlib()
        .partials(partials)
        .build().unwrap()
        .parse_file("templates/page.txt").unwrap();

    let globals = liquid::object!({
        "title": "Liquid",
        "name": "Foo Bar",
        "value": "some value",
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
}
