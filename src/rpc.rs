struct AppendEntriesArgs {
    term:           Term,
    leader_id:      Id,
    prev_log_index: Index,
    prev_log_term:  Term,
    entries:        Vec<LogEntry>,
    leader_commit:  Index,
}

struct AppendEntriesReturn {
    term:    Term,
    success: bool,
}

struct RequestVoteArgs {
    term:          Term,
    candidate_id:  Id,
    las_log_index: Index,
    last_log_term: Term,
}

struct RequestVoteReturn {
    term:         Term,
    vote_granted: bool,
}

trait RpcClient {
    fn encode_append_entries_args(AppendEntriesArgs) -> Vec<u8>;
    fn encode_request_vote_args(RequestVoteArgs) -> Vec<u8>;
    fn decode_append_entries_return(Vec<u8>) -> AppendEntriesReturn;
    fn decode_request_vote_return(Vec<u8>) -> RequestVoteReturn;

    fn append_entries(AppendEntriesArgs)-> AppendEntriesReturn {}
    fn request_vote(RequestVoteArgs) -> RequestVoteReturn {}
}

trait RpcServer {
    fn decode_append_entries_args(Vec<u8>) -> AppendEntriesArgs;
    fn decode_request_vote_args(Vec<u8>) -> RequestVoteArgs;
    fn encode_append_entries_return(AppendEntriesReturn) -> Vec<u8>;
    fn encode_request_vote_return(RequestVoteReturn) -> Vec<u8>;

    fn append_entries(AppendEntriesArgs)-> AppendEntriesReturn;
    fn request_vote(RequestVoteArgs) -> RequestVoteReturn;
}
