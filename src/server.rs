use super::{SERVER_HOST, SERVER_PORT};
use std::io::{TcpListener, TcpStream, Listener, Acceptor};
use std::io::stdio;

pub fn main() {
    match TcpListener::bind(SERVER_HOST, SERVER_PORT).listen() {
        Ok(mut acceptor) => {
            for mut stream in acceptor.incoming() {
                match stream {
                    Ok(stream) => { spawn(proc() {echo_stream(stream)}); },
                    Err(err)   => { println!("{}", err);                 },
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        },
    }
}

fn echo_stream(mut stream: TcpStream) {
    for b in stream.bytes() {
        match b {
            Ok(b)  => { print!("{}", b as char); },
            Err(e) => { println!("{}", e);       },
        }
        stdio::flush();
    }
}
