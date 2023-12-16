#[macro_use]
extern crate rouille;

fn main() {
    println!("Now listening on localhost:8000");
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::text("hello world")
            },
            _ => rouille::Response::empty_404()
        )
    });
}
