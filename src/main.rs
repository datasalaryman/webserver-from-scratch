use std::{net::{TcpListener, TcpStream}, io::{Write, Read, BufRead, BufReader}};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // enforces read within a nested block so that stream is preserved
    // TODO: figure how how to do a read from stream without cloning or
    // creating a nested block
    { 
        let reader = BufReader::new(&stream); 
        let lines = reader.lines(); 

        println!("===REQUEST===");

        for line in lines {
            let line_result = line?; 
            println!("{}", line_result);
            if line_result.is_empty() {
                break;  // headers done
            }
        }
    }

    println!("\r\n");

    println!("===RESPONSE===");

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 37\r\n\r\n<html><body>Hello World</body></html>";

    println!("{}", response); 

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok (())

}

fn main() -> std::io::Result<()> {
    // loop {
    //     println!("Hello, world!");
    // }
    let listener = TcpListener::bind("0.0.0.0:3000")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(e) => eprintln!("Accept error: {}", e),
        }
    }

    Ok(())
}
