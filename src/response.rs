use std::fs;
use crate::request::{Request}; 

#[derive(Debug)]
pub struct Header<'a> {
    pub key: &'a str, 
    pub value: &'a str
}

#[derive(Debug)]
pub struct Response<'a> {
    pub version: &'a str, 
    pub status: &'a u8,
    pub headers: Vec<Header<'a>>, 
    pub representation: Vec<Header<'a>>, 
    pub body: &'a str, 
}

#[derive(Debug)]
pub struct NotFoundError;

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nConnection: close")
    }
}

impl std::error::Error for NotFoundError {}

#[derive(Debug)]
pub struct InternalServerError;

impl std::fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
         write!(f, "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\nAllow: GET\r\nConnection: close")
    }
}

impl std::error::Error for InternalServerError {}

pub enum ResponseError {
    NotFoundError, 
    InternalServerError
}

impl<'a> TryFrom<&Request<'a>> for Response<'a> {

    type Error = ResponseError; 

    fn try_from(s: &Request<'a>) -> Result<Self, Self::Error> {
        let headers = Vec::<Header>::new();
        let representation = Vec::<Header>::new();
        let body = "<html><body>Hello World</body></html>";

        if s.route != "/" && s.route != "/favicon.ico" {
            return Err(ResponseError::NotFoundError)
        }
        
        Ok(Self {
            version: s.version, 
            status: &200, 
            headers, 
            representation, 
            body
        })
    }
}

impl<'a> std::fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let index = fs::read_to_string("assets/index.html").unwrap();
        write!(f, "{} {} OK\r\nContent-Type: text/html\r\nContent-Length: {:?}\r\n\r\n{}", 
            self.version, 
            self.status,
            index.len(), 
            index
        )
    }
}
