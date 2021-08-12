use crate::http::Request;
use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;

pub struct Server {
  addr: String,
}

impl Server {

    // new could be called any, but convention is to use "new" for constructor
    pub fn new(addr: String) -> Self {
        Server{
            addr // addr: addr
        }
    }

    // pass a reference to self, if we want run not taking ownership on the whole struct
    pub fn run(self) {
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }

                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                },
                // _ => println!("All variants that are not catched explicitly")
            }
        }
    }
}