fn main() {
    let result = render("direct: {% for item in items %}{{item}} {% endfor %}");
    println!("{}", result);

    let result = render("sorted: {% assign sorted = items | sort %}{% for item in sorted %}{{item}} {% endfor %}");
    println!("{}", result);
}

fn render(tmpl: &str) -> String {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(tmpl)
        .unwrap();

    let globals = liquid::object!({
        "items": vec![2, 8, 4, 6, 3, 5, 7],
    });
    template.render(&globals).unwrap()
}

#[test]
pub fn test_reverse() {
    let result = render("direct: {% for item in items %}{{item}} {% endfor %}");
    assert_eq!(result, "direct: 2 8 4 6 3 5 7 ");

    let result = render("sorted: {% assign sorted = items | sort %}{% for item in sorted %}{{item}} {% endfor %}");
    assert_eq!(result, "sorted: 2 3 4 5 6 7 8 ");
}
