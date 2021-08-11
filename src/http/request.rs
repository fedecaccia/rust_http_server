use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>, // Option enum wrapping the string, could be None or a String
    method: Method,
}