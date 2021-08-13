use crate::http::{ Request, Response, StatusCode, ParseError };
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    // new could be called any, but convention is to use "new" for constructor
    pub fn new(addr: String) -> Self {
        Server {
            addr, // addr: addr
        }
    }

    // pass a reference to self, if we want run not taking ownership on the whole struct
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // First option
            // let res = listener.accept();
            // if res.is_err() {
            //     continue;
            // }
            // let (stream, addr) = res.unwrap();

            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Listening on address {}", addr);
                    let mut buffer = [0; 1024]; // 1024 bytes with zeros
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // test reading using netcat:
                            // echo "\xFF TEST" | nc 127.0.0.1 8080
                            println!("Receive a request: {}", String::from_utf8_lossy(&buffer));

                            // let res: &Result<Request, _> = &buffer[..].try_into();
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                } // _ => println!("All variants that are not catched explicitly")
            }
        }
    }
}
