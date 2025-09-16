use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// Applies ROT13 transformation to a u8 slice.
/// Only ASCII letters are rotated; other bytes are unchanged.
pub fn rot13(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .map(|&b| match b {
            b'a'..=b'z' => (b - b'a' + 13) % 26 + b'a',
            b'A'..=b'Z' => (b - b'A' + 13) % 26 + b'A',
            _ => b,
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        println!("Waiting for a connection...");
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                println!("Getting data...");
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let received_text = String::from_utf8_lossy(&buf[0..n]);
                println!("Received text: {}", received_text);

                // Write the data back
                let encripted = rot13(&buf[0..n]);
                if let Err(e) = socket.write_all(&encripted[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
