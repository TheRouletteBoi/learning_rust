use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> anyhow::Result<()> {
    let server_address = "127.0.0.1:8080";

    let listener = TcpListener::bind(server_address)?;

    let (mut stream, socket_addr) = listener.accept()?;

    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                let client_message = String::from_utf8_lossy(&buffer);

                println!("client said {}", client_message);
            }

            Err(e) => {
                println!("error reading from client {}", e);
                break;
            }
        }
    }

    Ok(())
}
