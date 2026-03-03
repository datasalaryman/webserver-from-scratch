use std::{os::fd::RawFd, str::FromStr};
use nix::sys::socket::{
   AddressFamily, Backlog, MsgFlags, SockFlag, SockType, SockaddrIn, accept, bind, listen, recv, send, setsockopt, socket, sockopt::ReuseAddr
};
use nix::unistd::{close};
use std::os::fd::AsRawFd;
//use std::os::fd::AsFd;

fn handle_client(fd: RawFd) {
    
    let mut buf = [0u8; 1024];
    let n = recv(fd, &mut buf, MsgFlags::empty()).unwrap();

    if n == 0 {
        println!("Message empty"); 
    }

    
    println!("===REQUEST==="); 

    println!("{}", String::from_utf8_lossy(&buf[..n]));

    println!("\r\n");

    println!("===RESPONSE===");

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 37\r\n\r\n<html><body>Hello World</body></html>";

    println!("{}", response);

    send(fd, response.as_bytes().try_into().unwrap(), MsgFlags::empty()); 

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
