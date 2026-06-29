// Henry Boekhoff
// Little TCP Rust Server for learning
// Comments are for future Henry

use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Bind does normal bind things, unwrap()
    // returns a Result enum which is similar
    // to Haskell's Maybe type. Which is pretty cool.
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("Server waiting for connections...");

    // starts an infinite iterator that continuously accepts
    // connections.
    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                println!("New connection!");
                handle_connection(tcp_stream);
            }
            Err(e) => {
                println!("Connection failed!");
                println!("{}", e);
            }
        }
    }
}

fn handle_connection(stream: TcpStream) {
    // Creates a buffer reader with the tcp stream
    let buf_reader = BufReader::new(&stream);
    // Put lines of request from browser into a vector
    let http_request: Vec<_> = buf_reader
        // lines() creates an iterator of Result - splits data based on newlines
        .lines()
        // Get each line by mapping and unwrapping each Result
        .map(|result| result.unwrap())
        // Keep taking lines until double newline is found (aka empty string)
        .take_while(|line| !line.is_empty())
        // collect() collects. Iterators are lazy and need a terminator
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
