use std::io::{TcpListener, TcpStream, Listener, Acceptor};
use std::io::stdio;

pub fn serve(host: &str, port: u16) {
    println!("listening on {}:{}", host, port);
    match TcpListener::bind(host, port).listen() {
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
