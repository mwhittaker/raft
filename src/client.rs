use super::{SERVER_HOST, SERVER_PORT, CLIENT_PING_PERIOD};
use std::io::{TcpStream, Timer};

pub fn main() {
    let mut stream = TcpStream::connect(SERVER_HOST, SERVER_PORT);

    let mut timer = {
        match Timer::new() {
            Ok(timer) => timer,
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    };
    let periodic = timer.periodic(CLIENT_PING_PERIOD);

    loop {
        periodic.recv(); 
        match stream.write_str("hello\n") {
            Ok(()) => {},
            Err(e) => {println!("{}", e);},
        }
    }
}
