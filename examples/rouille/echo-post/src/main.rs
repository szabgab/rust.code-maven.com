#[macro_use]
extern crate rouille;

fn main() {
    let host = "localhost";
    let port = "8000";

    println!("Now listening on {host}:{port}");

    rouille::start_server(format!("{host}:{port}"), move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::html(r#"
                    Good form
                    <form method="POST" action="/echo">
                    <input name="text">
                    <input type="submit" value="Echo">
                    </form>

                    <hr>
                    Form with extra hidden field.
                    <form method="POST" action="/echo">
                    <input name="text">
                    <input name="other" type="hidden" value="42">
                    <input type="submit" value="Echo">
                    </form>

                    <hr>
                    Form where the expected field is missing.
                    <form method="POST" action="/echo">
                    <input name="other">
                    <input type="submit" value="Echo">
                    </form>
                    "#)
            },
            (POST) (/echo) => {
                let req = post_input!(request, {
                    text: String,
                });

                match req {
                    Ok(data) => {
                        println!("Received data: {:?}", data);
                        rouille::Response::html(format!(r#"You typed in <b>{}</b> <a href="/">back</a>"#, data.text))
                    },
                    Err(post_error) => {
                        println!("Received error: {:?}", post_error);
                        rouille::Response::html(format!(r#"error <b>{}</b>  <a href="/">back</a>"#, post_error)).with_status_code(400)
                    },
                }
            },
            _ => rouille::Response::html("This page does <b>not</b> exist.").with_status_code(404)
        )
    });
}
