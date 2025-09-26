
#[derive(serde::Serialize)]
enum Color {
    Blue,
    Green,
    Red,
}

fn main() {
    let template = r#"
      {% case color %}
        {% when "Blue" %}
            blue
        {% when "Green" %}
            green
        {% else %}
            Unrecognized color
      {% endcase %}
    "#;

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    // 1st
    let globals = liquid::object!({
        "color": Color::Blue,
    });
    let output = template.render(&globals).unwrap();
    println!("{output}");
    assert_eq!(output.trim(), "blue");

    // 2nd
    let globals = liquid::object!({
        "color": Color::Green,
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
    assert_eq!(output.trim(), "green");

    // 3rd
    let globals = liquid::object!({
        "color": Color::Red,
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
    assert_eq!(output.trim(), "Unrecognized color");
}
