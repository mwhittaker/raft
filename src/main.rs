use std::os;

pub mod server;
pub mod client;

static SERVER_HOST: &'static str = "127.0.0.1";
static SERVER_PORT: u16          = 9999;
static CLIENT_PING_PERIOD: u64   = 1000;

fn usage() {
    println!("raft (client|server)");
}

fn main() {
    let args = os::args();

    if args.len() < 2 {
        usage();
        os::set_exit_status(1);
        return;
    }

    match args[1].as_slice() {
        "server" => server::main(),
        "client" => client::main(),
        _        => usage(),
    };
}
