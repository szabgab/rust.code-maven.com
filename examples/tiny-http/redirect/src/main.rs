use ascii::AsciiString;
use std::str::FromStr;
use tiny_http::{Header, HeaderField, Response, Server, StatusCode};

fn main() {
    let host = "127.0.0.1";
    let port = "5000";

    let server_str = format!("{}:{}", host, port);

    let server = Server::http(&server_str).expect("Failed to start demo server.");
    println!("Visit http://{}", server_str);

    for request in server.incoming_requests() {
        let path = if let Some((path, _)) = request.url().split_once('?') {
            path
        } else {
            request.url()
        };

        println!("path: {}", path);

        let (html, status_code) = match path {
            "/" => root_page(&request),
            "/hello" => hello(&request),
            "/redir" => {
                redirect(request, "https://rust.code-maven.com/", 302);
                continue;
            }
            _ => default(&request),
        };

        let header = Header {
            field: HeaderField::from_str("Content-type").unwrap(),
            value: AsciiString::from_ascii("text/html").unwrap(),
        };

        request
            .respond(
                Response::from_string(html)
                    .with_status_code(StatusCode::from(status_code))
                    .with_header(header),
            )
            .unwrap();
    }
}

fn default(_request: &tiny_http::Request) -> (String, u32) {
    (String::from("Page not found"), 404)
}

fn redirect(request: tiny_http::Request, site: &str, status_code: u16) {
    let header = Header {
        field: HeaderField::from_str("Location").unwrap(),
        value: AsciiString::from_ascii(site).unwrap(),
    };
    request
        .respond(
            Response::from_string("")
                .with_status_code(StatusCode::from(status_code))
                .with_header(header),
        )
        .unwrap();
}

fn root_page(_request: &tiny_http::Request) -> (String, u32) {
    (
        String::from(
            r#"Welcome! Try <a href="/hello">this</a> page.  This is a <a href="/redir">redirect</a>"#,
        ),
        200,
    )
}

fn hello(_request: &tiny_http::Request) -> (String, u32) {
    (
        String::from(r#"Hello World! <a href="/">home</a> page."#),
        200,
    )
}
