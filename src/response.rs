use std::fs;
use crate::request::{Request}; 

#[derive(Debug)]
pub struct Header{
    pub key: String, 
    pub value: String 
}

#[derive(Debug)]
pub struct Response<'a> {
    pub version: &'a str, 
    pub status: &'a u8,
    pub headers: Vec<Header>, 
    pub representation: Vec<Header>, 
    pub body: String, 
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
        let mut representation = Vec::<Header>::new();
        if s.route != "/" 
            && s.route != "/favicon.ico" 
            && s.route != "/vite.svg"
            && s.route != "/assets/index-d526a0c5.css"
            && s.route != "/assets/index-908a9fcb.js"
            && s.route != "/assets/react-35ef61ed.svg"
        {
            return Err(ResponseError::NotFoundError)
        }
        let file_name = match s.route {
            "/" => "out/index.html".to_string(), 
            _ => format!("out{}", s.route), 
        };

        let body = fs::read_to_string(&file_name).unwrap();

        // println!("ROUTE: {} -> FILE: {}", s.route, &file_name.as_str()); 

        if s.route == "/" {
            representation.push(
                Header {
                    key: "Content-Type".to_string(), 
                    value: "text/html".to_string()
                }
            );
        } else if s.route.ends_with(".css") {
            representation.push(
                Header {
                    key: "Content-Type".to_string(), 
                    value: "text/css".to_string()
                }
            );
        } else if s.route.ends_with(".js") {
            representation.push(
                Header {
                    key: "Content-Type".to_string(), 
                    value: "application/javascript".to_string(),
                }
            );
        } else if s.route.ends_with(".svg") {
            representation.push(
                Header {
                    key: "Content-Type".to_string(), 
                    value: "image/svg+xml".to_string()
                }
            );
        };

        representation.push(
            Header {
                key: "Content-Length".to_string(), 
                value: body.len().to_string()
            }
        );

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

        let representation_headers = self.representation
                .iter()
                .map(|header| format!("{}: {}\r\n", header.key, header.value))
                .collect::<Vec<_>>()
                .join("");

        // println!("{}", representation_headers.as_str());

        write!(f, "{} {} OK\r\n{}\r\n{}", 
            self.version, 
            self.status,
            representation_headers.as_str(), 
            self.body
        )
    }
}
