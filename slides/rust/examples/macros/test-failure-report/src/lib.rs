pub fn calc(a: i32, b: i32) -> String {
    format!(r#"
    <html>
    <head>
    <title>Calc</title>
    </head>
    <body>
    <h1>Results</h1>
    <div id="add">{}</div>
    <div id="multiply">{}</div>
    </body>
    </html>
    "#, a*b, a+b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good() {
        let html = calc(2, 2);
        check_html_function(&html, "title", "Calc");
        check_html_function(&html, "h1", "Results");
        check_html_function(&html, "#add", "4");
        check_html_function(&html, "#multiply", "4");
        check_html_macro!(&html, "title", "Calc");
        check_html_macro!(&html, "h1", "Results");
        check_html_macro!(&html, "#add", "4");
        check_html_macro!(&html, "#multiply", "4");
    }

    #[test]
    fn test_bad_add_func() {
        let html = calc(3, 3);
        check_html_function(&html, "title", "Calc");
        check_html_function(&html, "#add", "6");
        check_html_function(&html, "h1", "Results");
    }


    #[test]
    fn test_bad_multiply_func() {
        let html = calc(3, 3);
        check_html_function(&html, "title", "Calc");
        check_html_function(&html, "#multiply", "9");
        check_html_function(&html, "h1", "Results");
    }

    #[test]
    fn test_bad_add_macro() {
        let html = calc(3, 3);
        check_html_macro!(&html, "title", "Calc");
        check_html_macro!(&html, "#add", "6");
        check_html_macro!(&html, "h1", "Results");
    }        

    #[test]
    fn test_bad_multiply_macro() {
        let html = calc(3, 3);
        check_html_macro!(&html, "title", "Calc");
        check_html_macro!(&html, "#multiply", "9");
        check_html_macro!(&html, "h1", "Results");
    }

    fn check_html_function(html: &str, css_selector: &str, expected: &str) {
            let document = scraper::Html::parse_document(html);
            let selector = scraper::Selector::parse(css_selector).unwrap();
            assert_eq!(
                &document.select(&selector).next().unwrap().inner_html(),
                expected
            );
    }
    
    macro_rules! check_html_macro {
        ($html: expr, $selectors: expr, $text: expr) => {{
            let document = scraper::Html::parse_document($html);
            let selector = scraper::Selector::parse($selectors).unwrap();
            assert_eq!(
                &document.select(&selector).next().unwrap().inner_html(),
                $text
            );
        }};
    }
    pub(crate) use check_html_macro;

}
