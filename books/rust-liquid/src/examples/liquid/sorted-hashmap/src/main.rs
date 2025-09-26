use std::collections::HashMap;


fn main() {
    let result = direct();
    println!("{}", result);

    let result = sort_by_key();
    println!("{}", result);
}


fn direct() -> String {
    render("direct: {% for pair in data %}{{pair[0]}}={{pair[1]}}; {% endfor %}")
}

fn sort_by_key() -> String {
    render("sorted: {% assign sorted = data.keys  %}{% for item in sorted %}{{item}} {% endfor %}")
}

fn render(tmpl: &str) -> String {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(tmpl)
        .unwrap();

    let data = HashMap::from([("dog", 1), ("cat", 3), ("snake", 2)]);

    let globals = liquid::object!({
        "data": data,
    });
    template.render(&globals).unwrap()
}

#[test]
pub fn test_reverse() {
    // Cannot test as the order is arbitrary and changes between runs
    //let result = direct();
    //assert_eq!(result, "direct: snake=2; dog=1; cat=3; ");

    //let result = render("sorted: {% assign sorted = items | sort %}{% for item in sorted %}{{item}} {% endfor %}");
    //assert_eq!(result, "sorted: 2 3 4 5 6 7 8 ");
}
