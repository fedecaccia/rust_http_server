use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf_lifetime> {
    path: &'buf_lifetime str,
    query_string: Option<&'buf_lifetime str>, // Option enum wrapping the string, could be None or a String
    method: Method,
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

impl<'buf_lifetime> TryFrom<&'buf_lifetime [u8]> for Request<'buf_lifetime> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n....HEADERS....

    fn try_from(buf: &'buf_lifetime [u8]) -> Result<Request<'buf_lifetime>, Self::Error> {
        // OPTION 1
        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // OPTION 2
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e)
        // }

        // OPTION 3
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        // OPTION 4
        let request = str::from_utf8(buf)?; // looks for Utf8Error parser, implemented in this file

        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        // creates a new "request" variable: variable shadowing
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // OPTION 1

        // match path.find('?') {
        //     Some(i) => {
        //         // we know that '?' is 1 byte long
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i]
        //     },
        //     None => {}
        // }

        // OPTION 2

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        // OPTION 3

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word<'a>(request: &'a str) -> Option<(&'a str, &'a str)> {
    // returns (word, rest_of_string) or None
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {},
    //         None => break
    //     }
    // }

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // we know that space is 1 byte long
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }
