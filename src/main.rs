use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    //get request and dump
    let mut request = [0; 1024];
    stream.read(&mut request).unwrap();
    println!("Received HTTP Request...\n{}", String::from_utf8_lossy(&request[..]));
    let response;
    if request.starts_with(b"GET / HTTP/1.1\r\n") {
        //send hello world response
        response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=iso-8859-1\r\nConnection: close\r\nContent-Length: 14\r\n\r\nHello world!\r\n";
    } else {
        response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=iso-8859-1\r\nConnection: close\r\nContent-Length: 4\r\n\r\nNope\r\n";
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn main() -> std::io::Result<()> {
    let server = "127.0.0.1:1024";
    let listener = TcpListener::bind(server)?;

    // accept connections and process them serially
    println!("Server is running on {}...", server);
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
