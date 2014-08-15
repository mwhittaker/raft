use super::{CLIENT_PING_PERIOD};
use std::io::timer;
use std::time::{Duration};
use std::io::{TcpStream, Timer};

fn get_stream(host: &str, port: u16) -> TcpStream {
    println!("connecting to {}:{}", host, port);
    loop {
        match TcpStream::connect(host, port) {
            Ok(stream) => {
                return stream;
            },
            Err(_) => {
                timer::sleep(Duration::seconds(1));
            }
        }
    }
}

pub fn connect(host: &str, port: u16, msg: &str) {
    let mut stream = get_stream(host, port);

    let mut timer = {
        match Timer::new() {
            Ok(timer) => timer,
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    };
    let periodic = timer.periodic(Duration::milliseconds(CLIENT_PING_PERIOD));

    loop {
        periodic.recv();
        match stream.write_str(msg) {
            Ok(()) => {},
            Err(e) => {println!("{}", e);},
        }
    }
}
