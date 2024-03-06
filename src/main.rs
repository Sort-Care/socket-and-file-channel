use std::{
    io::{Read, Write},
    net::TcpListener,
};
use std::{str, thread};

fn main() {
    println!("Hello, world!");
    start_socket_server();
}

fn handle_connection(mut stream: std::net::TcpStream) {
    match stream.peer_addr() {
        Ok(addr) => println!("Connection established with {}", addr),
        Err(e) => eprintln!("Couldn't get peer address: {}", e),
    }
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => match str::from_utf8(&buffer[..size]) {
            Ok(message) => {
                let response = format!("received: {} bytes, message: {}", size, message);
                match stream.write_all(response.as_bytes()) {
                    Ok(_) => {
                        println!("{}", response);
                        true
                    },
                    Err(e) => {
                        println!("Error when trying to response: {}", e);
                        false
                    },
                }
            }
            Err(e) => {
                println!("Error converting buffer to string: {}", e);
                false
            }
        },
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn start_socket_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Socket server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("Error establishing connection: {}", e)
            }
        }
    }
}
