fn main() {
    let template = "
        {% for color in colors %}
           {{color}}
        {% endfor %}
    ";

    let template = liquid::ParserBuilder::with_stdlib()
        .build().unwrap()
        .parse(template).unwrap();

    //let colors: [&str; 3] = ["red", "green", "blue"];
    let colors = vec!["red", "green", "blue"];

    let globals = liquid::object!({
        "colors": colors,
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
}
