fn main() {
    plain_text();
    embed_html();
    escape_html();
}

fn plain_text() {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse("<h1>Welcome to {{field}}</h1>").unwrap();

    let globals = liquid::object!({
        "field": "Liquid"
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
    assert_eq!(output, "<h1>Welcome to Liquid</h1>");
}


fn embed_html() {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse("<h1>Welcome to {{field}}</h1>").unwrap();

    let globals = liquid::object!({
        "field": "<>"
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
    assert_eq!(output, "<h1>Welcome to <></h1>");
}


fn escape_html() {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse("<h1>Welcome to {{field | escape}}</h1>").unwrap();


    let globals = liquid::object!({
        "field": "<>"
    });
    let output = template.render(&globals).unwrap();
    println!("{}", output);
    assert_eq!(output, "<h1>Welcome to &lt;&gt;</h1>");
}
