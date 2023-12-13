use ascii::AsciiString;
use std::collections::HashMap;
use std::str::FromStr;
use tiny_http::{Header, HeaderField, Response, Server};

fn main() {
    let host = "127.0.0.1";
    let port = "5000";

    let server_str = format!("{}:{}", host, port);

    let server = Server::http(&server_str).expect("Failed to start demo server.");

    for request in server.incoming_requests() {
        let header = Header {
            field: HeaderField::from_str("Content-type").unwrap(),
            value: AsciiString::from_ascii("text/html").unwrap(),
        };

        let mut html = String::from(
            r#"<h1>Echo</h1>
        <form>
        <input type="text" name="text">
        <input type="submit" value="Echo">
        </form>
        "#,
        );

        let params = get_url_parameters(&request);
        // println!("params: {:?}", params);

        if params.contains_key("text") {
            let text = &params["text"][0];
            if !text.is_empty() {
                html.push_str(format!("You typed: '{}'", text).as_str());
            }
        }

        request
            .respond(Response::from_string(html).with_header(header))
            .unwrap();
    }
}

fn get_url_parameters(request: &tiny_http::Request) -> HashMap<String, Vec<String>> {
    // TODO get rid of this fake URL thing
    let fake_url = format!("http://localhost{}", request.url().to_owned());
    // println!("full_url: {fake_url}");

    let req = url::Url::parse(&fake_url).unwrap();
    // println!("path: {}", req.path());
    let mut query: HashMap<String, Vec<String>> = HashMap::new();

    for (field, value) in req.query_pairs() {
        let field = field.to_string();
        let value = value.to_string();
        if !query.contains_key(&field) {
            query.insert(field.clone(), vec![]);
        }

        query
            .entry(field)
            .and_modify(|vector| vector.push(value.to_owned()));
    }

    query
}
