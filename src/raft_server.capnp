@0xe73b3a3c9ecd957f;

interface RaftServer {

  appendEntries @0(term :Int64, leaderId :Text, prevLogIndex :Int64, prevLogTerm :Int64, entries :List(LogEntry), leaderCommit :Int64) -> (term :Int64, success :Bool)

  struct LogEntry {
    value @0 :Text;
  }

  requestVote @1(term :Int64, candidateId :Text, lastLogIndex :Int64, lastLogTerm :Int64) -> (term :Int64, voteGranted :Bool)
  
}




