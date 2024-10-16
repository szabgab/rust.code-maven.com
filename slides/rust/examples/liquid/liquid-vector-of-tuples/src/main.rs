fn main() {
    let template = "
        {% for color in colors %}
           {{ color[0] }} - {{ color[1] }}
        {% endfor %}
    ";

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let colors = vec![("red", 23), ("green", 17), ("blue", 42)];

    let globals = liquid::object!({
        "colors": colors,
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
}
