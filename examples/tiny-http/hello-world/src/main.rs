use tiny_http::{Response, Server};

fn main() {
    let host = "127.0.0.1";
    let port = "5000";

    let server_str = format!("{}:{}", host, port);

    let server = Server::http(&server_str).expect("Failed to start demo server.");

    for request in server.incoming_requests() {
        request
            .respond(Response::from_string("Hello World!"))
            .unwrap();
    }
}

