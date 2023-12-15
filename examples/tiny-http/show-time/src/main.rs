use ascii::AsciiString;
use chrono::{DateTime, Local, Utc};
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
        let utc: DateTime<Utc> = Utc::now();
        let local: DateTime<Local> = Local::now();
        let html = format!("<table><tr><td>UTC: </td><td>{}</td></tr><tr><td>Localtime: </td><td>{}</td></tr></table>", utc, local);
        request
            .respond(Response::from_string(html).with_header(header))
            .unwrap();
    }
}
