
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = std::fs::read_to_string("content.md")?;
    let html = markdown2html(&markdown)?;
    println!("{html:?}");
    Ok(())
}

fn markdown2html(content: &str) -> Result<String, markdown::message::Message> {
    let html = markdown::to_html_with_options(
        content,
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                //allow_dangerous_protocol: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::gfm()
        },
    )?;

    Ok(html)
}


