#[macro_use]
extern crate rouille;

fn main() {
    let host = "localhost";
    let port = "8000";

    println!("Now listening on {host}:{port}");

    rouille::start_server(format!("{host}:{port}"), move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::html("Hello <b>world!</b>")
            },
            _ => rouille::Response::html("This page does <b>not</b> exist.").with_status_code(404)
        )
    });
}
