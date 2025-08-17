fn main() {
    let template = include_str!("../template.txt");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let name = String::from("Liquid");
    let globals = liquid::object!({
        "name": name
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
}
