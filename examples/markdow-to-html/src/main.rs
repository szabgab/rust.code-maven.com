fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = std::fs::read_to_string("content.md")?;
    match markdown2html(&markdown) {
        Ok(html) => println!("{html}"),
        Err(err) => return Err(Box::<dyn std::error::Error>::from(format!("{err}"))),
    }

    Ok(())
}

fn markdown2html(content: &str) -> Result<String, markdown::message::Message> {
    let html = markdown::to_html_with_options(
        content,
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::gfm()
        },
    )?;

    Ok(html)
}
