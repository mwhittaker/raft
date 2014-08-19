trait Marshallable {
    fn marshal(self) -> Vec<u8>;
    fn unmarshal(Vec<u8>) -> Option<(Vec<u8>, Self)>;
}
