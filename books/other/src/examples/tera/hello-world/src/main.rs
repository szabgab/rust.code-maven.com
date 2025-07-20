use tera::Context;
use tera::Tera;

fn main() {
    let tera = Tera::new("templates/*.html").unwrap();

    let mut context = Context::new();
    context.insert("name", "World");

    let result = tera.render("hello.html", &context).unwrap();
    assert_eq!(result, "Hello, World!");
    println!("{result}");
}
