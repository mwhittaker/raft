type Index = u64;

struct LogEntry<T> {
    command: T,
    term: Term,
}
