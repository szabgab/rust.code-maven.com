use tera::Tera;

fn main() {
    let tera = Tera::new("templates/*.html").unwrap();

    tera.get_template_names().for_each(|x| println!("{}", x));
}
