use std::os;
use std::io::{TcpListener, TcpStream, Listener, Acceptor};

type Address = (String, u16);

fn usage() -> &'static str {
    "raft bind_address [connect addresses]..."
}

fn main() {
    let mut args = os::args().move_iter();
    
    let _ /* executable name */ = args.next();
    let bind_address = args.next().expect(usage());    
    let connect_addresses = args.collect();
    listen_and_connect(bind_address, connect_addresses);
}

fn listen_and_connect(bind_address: String, connect_addresses: Vec<String>) {
    let bind_address = parse_address(bind_address).expect("couldn't parse bind address");
    let connect_addresses = connect_addresses.move_iter().map(|s| {
        parse_address(s).expect("couldn't parse listen address")
    }).collect();
    lac(bind_address, connect_addresses);
}

fn lac(bind_address: Address, connect_addresses: Vec<Address>) {
    //spawn(proc() {serve(bind_address)});
    //spawn(proc() {connect(connect_addresses)});
}

fn parse_address(address: String) -> Option<Address> {
    let hostport: Vec<&str> = address.as_slice().split(':').collect();
    if hostport.len() < 2 {
        None
    } else {
        let host = hostport[0];
        let port: u16 = match from_str(hostport[1]) {
            Some(port) => port,
            None => {
                println!("could not parse port");
                return None;
            }
        };
        Some((String::from_str(host), port))
    }
}

#[test]
fn foo() {
    fn assert(expected: Option<Address>, actual: &str) {
        let actual = parse_address(String::from_str(actual));
        assert!(expected == actual);
    }

    assert(None, "");
    assert(None, "localhost");
    assert(None, "8080");
    assert(None, "localhost8080");
    assert(None, "localhost:");
    assert(Some((String::from_str("localhost"), 8080u16)), "localhost:8080");
}
