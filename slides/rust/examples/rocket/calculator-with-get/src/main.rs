#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/?<a>&<b>&<op>")]
fn index(a: Option<i64>, b: Option<i64>, op: Option<String>) -> content::RawHtml<String> {
    let mut selected_add = "";
    let mut selected_multiply = "";
    let mut selected_subtract = "";
    let mut selected_divide = "";

    let result = match (a, b, op) {
        (Some(a), Some(b), Some(op)) => match op.as_ref() {
            "add" => {
                selected_add = r#"selected="selected""#;
                (a + b).to_string()
            }
            "subtract" => {
                selected_subtract = r#"selected="selected""#;
                (a - b).to_string()
            }
            "divide" => {
                selected_divide = r#"selected="selected""#;
                (a / b).to_string()
            }
            "multiply" => {
                selected_multiply = r#"selected="selected""#;
                (a * b).to_string()
            }
            _ => String::new(),
        },
        _ => String::new(),
    };

    let a = match a {
        Some(a) => a.to_string(),
        None => String::new(),
    };

    let b = match b {
        Some(b) => b.to_string(),
        None => String::new(),
    };

    let mut html = format!(
        r#"
    <form>
    <input name="a" value="{a}">
    <input name="b" value="{b}">
    <select name="op">
    <option value="add" {selected_add}>+</option>
    <option value="multiply" {selected_multiply}>*</option>
    <option value="subtract" {selected_subtract}>-</option>
    <option value="divide" {selected_divide}>/</option>
    </select>

    <input type="submit" value="Calculate">
    "#
    );

    if !result.is_empty() {
        let res_html = format!("<hr>The result is {result}");
        html.push_str(&res_html);
    }

    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
