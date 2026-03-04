use std::{os::fd::RawFd, str::FromStr};
use nix::sys::socket::{
   AddressFamily, Backlog, MsgFlags, SockFlag, SockType, SockaddrIn, accept, bind, listen, recv, send, setsockopt, socket, sockopt::ReuseAddr
};
use nix::unistd::{close};
use std::os::fd::AsRawFd;

#[derive(Debug)]
struct Header<'a> {
    key: &'a str, 
    value: &'a str
}

#[derive(Debug)]
struct ParsedRequest<'a> {
    method: &'a str,
    route: &'a str,
    version: &'a str,
    headers: Vec<Header<'a>>, 
}

fn parse_request_message (lines: &[String]) -> Result<ParsedRequest, String> {
    
    let start_line: Vec<&str> = lines[0].split(" ").collect();
    if start_line.len() < 2 {
        return Err(String::from("Length is wrong"));
    }
    // Only allow GET methods for now
    if start_line[0] != "GET" {
        return Err(String::from("Method is wrong"));
    }
    if start_line[1] != "/" 
       && start_line[1] != "/favicon.ico" 
    {
        return Err(String::from(format!("Route {} is not allowed", start_line[1])));
    }
    if start_line[2] != "HTTP/1.1" {
        return Err(String::from("Version is wrong"));
    }

    let mut headers_lines: Vec<&str> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            headers_lines = lines[1..i-1].iter().map(|v| v.as_str()).collect();
            break;
        } 
    }

    let mut headers = Vec::new(); 
    for header_line in &headers_lines {
        let (k, v) = header_line.split_once(": ").unwrap(); 
        headers.push(Header {key: k, value: v});
    }
    
    Ok(ParsedRequest {
        method: &start_line[0], 
        route: &start_line[1], 
        version: &start_line[2],
        headers 
    })
}

fn handle_client(fd: RawFd) -> () {
    
    let mut buf = [0u8; 1024];
    let n = recv(fd, &mut buf, MsgFlags::empty()).unwrap();

    let mut lines = Vec::new();

    if n == 0 {
        println!("Message empty"); 
    }

    let mut segment_start = 0;

    for (i, _byte) in buf.iter().enumerate() {
        if i + 1 < n && 
            [buf[i], buf[i+1]] == [0x0D, 0x0A] {
            lines.push(String::from_utf8_lossy(&buf[segment_start..i]).to_string()); 
            segment_start = i + 2;
        }
    }

    let message = parse_request_message(&lines);

    let success_response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 37\r\n\r\n<html><body>Hello World</body></html>";

    let fail_response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nConnection: close";

    match message {
        Ok(_ParsedRequest) => send(fd, success_response.as_bytes().try_into().unwrap(), MsgFlags::empty()), 
 
        Err(_e) => send(fd, fail_response.as_bytes().try_into().unwrap(), MsgFlags::empty()), 

    };

}

fn main() {
    let sock_addr = SockaddrIn::from_str("0.0.0.0:3000").unwrap(); 

    let fd = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None).unwrap();

    setsockopt(&fd, ReuseAddr, &true).unwrap();

    bind(fd.as_raw_fd(), &sock_addr).unwrap();

    listen(&fd, Backlog::new(128).unwrap()).unwrap();

    loop {
        let client_fd = accept(fd.as_raw_fd()).unwrap();

        println!("IPV4 Address: {:?}, Port: {:?}", &sock_addr.ip(), &sock_addr.port());
        println!("File descriptor: {:?}", &fd);
        handle_client(client_fd);

        close(client_fd).unwrap(); 
    }

}
