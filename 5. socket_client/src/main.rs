use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let server_address = "127.0.0.1:8080";

    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("You are connected to {}", server_address);

            loop {
                let to_send = "hello there";
                stream
                    .write(to_send.as_bytes())
                    .expect("failed to create fmt");

                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let response = String::from_utf8_lossy(&buffer);
                        println!("Server Responded with: {}", response);
                    }

                    Ok(_) => {
                        println!("The server connection closed");
                        break;
                    }

                    Err(e) => {
                        println!("Failed reading from server {}", e);
                        break;
                    }
                }
            }
        }

        Err(e) => {
            println!("Failed to connect to server {}", e);
        }
    }
}
