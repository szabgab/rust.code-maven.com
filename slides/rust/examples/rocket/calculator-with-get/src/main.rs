#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/?<a>&<b>")]
fn index(a: Option<i64>, b: Option<i64>) -> content::RawHtml<String> {
    let result = match (a, b) {
        (Some(a), Some(b)) => (a + b).to_string(),
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
    <input name="a" value="{a}"> <input name="b" value="{b}">
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
