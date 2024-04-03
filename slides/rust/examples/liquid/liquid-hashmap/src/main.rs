use std::collections::HashMap;

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

    let colors: HashMap<&str, u32> = HashMap::from([("red", 23), ("green", 17), ("blue", 42)]);
    println!("{colors:#?}");

    let globals = liquid::object!({
        "colors": colors,
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
}
