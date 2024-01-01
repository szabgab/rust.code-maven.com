#[macro_use]
extern crate rouille;

fn main() {
    let host = "localhost";
    let port = "8000";

    println!("Now listening on {host}:{port}");

    rouille::start_server(format!("{host}:{port}"), move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::text("Hello <b>world!</b>")
            },
            _ => rouille::Response::empty_404()
        )
    });
}
