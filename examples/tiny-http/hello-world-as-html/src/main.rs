use ascii::AsciiString;
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
        request
            .respond(Response::from_string("<h1>Hello World</h1>").with_header(header))
            .unwrap();
    }
}

