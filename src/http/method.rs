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