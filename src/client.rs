struct Client<T>;

impl<T: Marshallable> Client<T> {
    fn new(host: &str, port: u16) -> IoResult<Client<T>> {}
    fn write(entry: T) -> IoResult<()> {}
}
