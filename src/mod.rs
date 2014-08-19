mod server;
mod client;
mod rpc;
mod log;

trait StateMachine<T> {
    fn new() -> Self;
    fn f(&mut self, iter: Iterator<T>);
}

trait Raft<T, M> {
    fn New(Vec<SocketAddr>) -> Raft<T>;
    fn write(t: T) -> RafResult<()>;
}

impl<T: Serializable, M: StateMachine<T>> Raft<T, M> for Raft {

}
