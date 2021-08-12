use std::str::FromStr;

pub enum Method { // in memory represented by numbers, starging from 0
  // GET(String), // this one is defined by a String
  GET,
  DELETE,
  POST, // POST=5, set an enum number, any one from this one, get increased values
  PUT,
  HEAD,
  CONNECT,
  OPTIONS,
  TRACE,
  PATCH
}

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "DELETE" => Ok(Self::DELETE),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "HEAD" => Ok(Self::HEAD),
      "CONNECT" => Ok(Self::CONNECT),
      "OPTIONS" => Ok(Self::OPTIONS),
      "TRACE" => Ok(Self::TRACE),
      "PATCH" => Ok(Self::PATCH),
      _ => Err(MethodError)
    }
  }
}

pub struct MethodError;
