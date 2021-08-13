use std::net::TcpStream;
use std::io::{ Write, Result as IoResult };
use std::fmt::{Display, Formatter, Result as FmtResult};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // pub fn send_TcpStream(&self, stream: &mut impl TcpStream) -> IoResult<()> {
    // pub fn send_File(&self, stream: &mut impl File) -> IoResult<()> {
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> { // accepts any parameter that implements the trait Write
      let body = match &self.body {
        Some(b) => b,
        None => "",
      };

      write!(
          stream,
          "HTTP/1.1 {} {}\r\n\r\n{}",
          self.status_code,
          self.status_code.reason_phrase(),
          body
      )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
