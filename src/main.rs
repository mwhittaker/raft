use std::os;
use help::{usage, parse_address};

type Address = (String, u16);

fn main() {
    let mut args = os::args().move_iter();
    
    let _          = args.next();
    let bind_addr  = args.next().expect(usage());
    let conn_addrs = args.collect();
    network(bind_addr, conn_addrs);
}

fn network(bind_addr: String, conn_addrs: Vec<String>) {
    let bind_addr = parse_address(bind_addr).expect(usage());
    let conn_addrs = conn_addrs.move_iter().map(|s| {
        parse_address(s).expect(usage())
    }).collect();
    
    _network(bind_addr, conn_addrs);
}

fn _network(bind_addr: Address, conn_addrs: Vec<Address>) {
    let _ = bind_addr;
    let _ = conn_addrs;
    //spawn(proc() {serve(bind_addr)});
    //spawn(proc() {connect(conn_addrs)});
}

mod help {
    pub fn usage() -> &'static str {
        "raft bind_addr [connect addresses]..."
    }

    pub fn parse_address(address: String) -> Option<super::Address> {
        let hostport: Vec<&str> = address.as_slice().split(':').collect();
        
        if hostport.len() < 2 {
            return None
        } 

        match (String::from_str(hostport[0]), from_str(hostport[1])) {
            (host, Some(port)) => Some((host, port)),
            (_   , None      ) => None,
        }
    }
}

#[test]
fn foo() {
    fn assert(expected: Option<Address>, actual: &str) {
        assert_eq!(expected, parse_address(String::from_str(actual)));
    }

    let from = String::from_str;

    assert(None, "");
    assert(None, "localhost");
    assert(None, "8080");
    assert(None, "localhost8080");
    assert(None, "localhost:");
    assert(Some((from("localhost"), 8080u16)), "localhost:8080");
    assert(Some((from("localhost"), 0u16)), "localhost:0");
    assert(Some((from(""), 0u16)), ":0");
}
