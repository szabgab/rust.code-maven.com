use tera::Context;
use tera::Tera;

fn main() {
    let tera = Tera::new("templates/*.html").unwrap();

    let mut context: Context = Context::new();
    context.insert("text", " Hello, World! How are you? ");

    let result = tera.render("hello.html", &context).unwrap();
    println!("{result}");
}
