fn main() {
    let text = "This is some long text that we need to wrap to fit in a given width".to_string();
    println!("{}", text);
    let lines = textwrap::wrap(&text, 28);
    println!("{:#?}", lines);

    let lines = textwrap::wrap(&text, 4);
    println!("{:#?}", lines);
}
