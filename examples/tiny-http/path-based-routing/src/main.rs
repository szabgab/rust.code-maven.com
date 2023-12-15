use ascii::AsciiString;
use std::str::FromStr;
use tiny_http::{Header, HeaderField, Response, Server};

fn main() {
    let host = "127.0.0.1";
    let port = "5000";

    let server_str = format!("{}:{}", host, port);

    let server = Server::http(&server_str).expect("Failed to start demo server.");

    for request in server.incoming_requests() {
        let path = if let Some((path, _)) = request.url().split_once('?') {
            path
        } else {
            request.url()
        };

        println!("path: {}", path);

        let html = match path {
            "/" => root_page(&request),
            "/hello" => hello(&request),
            _ => default(&request),
        };

        let header = Header {
            field: HeaderField::from_str("Content-type").unwrap(),
            value: AsciiString::from_ascii("text/html").unwrap(),
        };

        request
            .respond(Response::from_string(html).with_header(header))
            .unwrap();
    }
}

fn default(_request: &tiny_http::Request) -> String {
    String::from("Page not found")
}

fn root_page(_request: &tiny_http::Request) -> String {
    String::from(r#"Welcome! Try <a href="/hello">this</a> page."#)
}

fn hello(_request: &tiny_http::Request) -> String {
    String::from(r#"Hello World! <a href="/">home</a> page."#)
}
