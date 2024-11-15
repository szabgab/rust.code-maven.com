#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/?<a>&<b>&<op>")]
fn index(a: Option<i64>, b: Option<i64>, op: Option<&str>) -> content::RawHtml<String> {
    let mut selected_add = "";
    let mut selected_multiply = "";
    let mut selected_subtract = "";
    let mut selected_divide = "";

    let result = match (a, b, op) {
        (Some(a), Some(b), Some(op)) => match op {
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

#[cfg(test)]
mod test {
    use super::rocket;

    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_no_input() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(!html.contains("result"));

        assert!(html.contains(r#"<form>"#));
        assert!(html.contains(r#"<input name="a" value="">"#));
        assert!(html.contains(r#"<input name="b" value="">"#));
        assert!(html.contains(r#"<select name="op">"#));
        assert!(html.contains(r#"<option value="add" >+</option>"#));
        assert!(html.contains(r#"<option value="multiply" >*</option>"#));
        assert!(html.contains(r#"<option value="subtract" >-</option>"#));
        assert!(html.contains(r#"<option value="divide" >/</option>"#));
        assert!(html.contains(r#"</select>"#));
        assert!(html.contains(r#"<input type="submit" value="Calculate">"#));
    }

    #[test]
    fn test_add() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/?a=23&b=19&op=add").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains("<hr>The result is 42"));
        assert!(html.contains(r#"<input name="a" value="23">"#));
        assert!(html.contains(r#"<input name="b" value="19">"#));
    }

    #[test]
    fn test_missing_b() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/?a=23&op=add").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(!html.contains("result"));
        assert!(html.contains(r#"<input name="a" value="23">"#));
        assert!(html.contains(r#"<input name="b" value="">"#));

        //assert_eq!(html, "");
        // TODO: maybe this should report an error?
    }
}
