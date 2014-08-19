use std::io::{TcpListener, TcpStream, Listener, Acceptor};
use std::io::stdio;

type Term = u64;
type Id = SocketAddr;

struct PersistentState {
    current_term: Term,
    voted_for:    Option<Term>,
    log:          Vec<LogEntry>,
}

struct VolatileStateAll {
    commit_index: Index,
    last_applied: Index,
}

struct VolatileStateLeader {
    next_index:  Vec<Index>,
    match_index: Vec<Index>,
}

struct Server {
    persistent_state:      PersistentState,
    volatile_state_all:    VolatileStateAll,
    volatile_state_leader: Option<VolatileStateLeader>,
}
