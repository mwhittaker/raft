use std::io::{IoResult, TcpListener, TcpStream, Listener, Acceptor};

pub fn main() {
    match exec() {
        Ok(()) => {},
        Err(e) => println!("{}", e),
    }
}

fn exec() -> IoResult<()> {
    let     listener = try!(TcpListener::bind(super::SERVER_HOST, super::SERVER_PORT));
    let mut acceptor = try!(listener.listen());
    for mut stream in acceptor.incoming() {
        match stream {
            Ok(stream) => {
                spawn(proc() {echo_stream(stream)});
            },
            Err(e) => {
                println!("{}", e);
            },
        }
    }
    Ok(())
}

fn echo_stream(mut stream: TcpStream) {
    loop {
        match stream.read_byte() {
            Ok(b) => {
                let c = b as char;
                if c == '\n' {
                    println!("");
                } else {
                    print!("{}", c);
                }
            },
            Err(e) => {
                println!("{}", e);
                return
            },
        }
    }
}
