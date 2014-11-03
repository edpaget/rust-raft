use std::collections::HashMap;
use url::Url;
use std::io::timer::Timer;
use std::time::Duration;

use raft_server_capnp::raft_server::log_entry;

mod raft::rpc;

enum State {
    Follower,
    Candidate,
    Leader,
}

struct ClusterState<'a>(HashMap<&'a str,Url>);

struct LeaderState<'a> {
    next_index: HashMap<&'a str,u64>,
    match_index: HashMap<&'a str,u64>,
}

struct ServerState<'a> {
    state: State,
    current_term: u64,
    voted_for: Option<u64>,
    log: Vec<&'a str>,
    commit_index: u64,
    last_applied: u64,
    cluster_state: ClusterState<'a>,
    leader_state: Option<LeaderState<'a>>,
}

impl ServerState {
    fn append_entries(&mut self,
                      term : u64,
                      leader_id: &str,
                      prev_log_index : u64,
                      prev_log_term: u64,
                      entries : log_entry::Reader,
                      leader_commit : u64) -> Result<(u64, bool), String> {
        match cmp(self.current_term, term) {
            Less => Ok(self.current_term, false),
            Greater => ,
            Equal => ,
        }
        
        
    }

    fn request_vote(&mut self,
                    term : u64,
                    candidate_id: &str,
                    last_log_index: u64,
                    last_log_term: u64) -> Result<(u64, bool), String> {
    }
}


pub fn start_server() {
    let mut server = ServerState{
        state: Follower,
        current_term: 0,
        voted_for: None,
        log: Vec::new(),
        commit_index: 0,
        last_applied: 0,
        cluster_state: ClusterState(HashMap::new()),
        leader_state: None,
    };

    let (tx, rx) = channel::<int>();
    let mut timer = Timer::new().unwrap();

    loop {
        let timeout = timer.oneshot(Duration::milliseconds(150));

        select! {
            val = rx.recv() => println!("HERE"),
            () = timeout.recv() => {
                server.state = Candidate;
                println!("No RPCs recieved");
            }
        }
    }
}
