use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    println!("address: {:?}", listener);
    println!("port: {}", listener.local_addr().unwrap().port());
}
