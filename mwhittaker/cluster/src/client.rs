use std::io::TcpStream;

pub static ADDRESS : &'static str = "127.0.0.1";
pub static PORT    : u16          = 8080;

fn main() {
    TcpStream::connect(ADDRESS, PORT)
    .map(|mut stream| {
        stream.read_to_str().map(|msg| {println!("{}", msg)})
    });
}
