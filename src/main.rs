use std::os;

pub mod server;
pub mod client;

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
