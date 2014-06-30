use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};

pub static ADDRESS : &'static str = "127.0.0.1";
pub static PORT    : u16          = 8080;

fn handle_stream(mut stream: TcpStream) {
    stream.write_str("hello");
}

fn main() {
    let res = TcpListener::bind(ADDRESS, PORT)
    .map(|listener| { listener.listen() })
    .map(|mut acceptor| {
        for stream in acceptor.incoming() {
            stream.map(|stream| { 
                spawn( proc(){handle_stream(stream)} );
            });
        }
    });

    println!("{}", res);
}
