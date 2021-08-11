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

        let listener = TcpListener::bind(&self.addr);
    }
}