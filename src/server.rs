use crate::http::request::Request;
use std::{convert::TryFrom, io::Read, net::TcpListener};

pub struct Server {
    addr: String,
    port: u16,
}

impl Server {
    pub fn new(addr: String, port: u16) -> Self {
        Self { addr, port }
    }

    pub fn run(self) {
        println!("Server is running -> {}:{}", self.addr, self.port);
        let full_address = self.addr + ":" + self.port.to_string().as_str();

        let server = TcpListener::bind(full_address).unwrap();

        loop {
            match server.accept() {
                Ok((mut stream, addr)) => {
                    println!("Accepting connection from {}", addr);

                    let mut buff = [0; 1024];
                    match stream.read(&mut buff) {
                        Ok(_) => {
                            println!(
                                "Reading from connection:\n{}",
                                String::from_utf8_lossy(&buff)
                            );
                            match Request::try_from(&buff[..]) {
                                Ok(request) => {}
                                Err(error) => println!("Failed to parse request: {}", error),
                            }
                        }
                        Err(error) => println!("Failed to read from connection: {}", error),
                    }
                }
                Err(error) => {
                    println!("Failed to established connection: {}", error);
                    continue;
                }
            }
        }
    }
}
