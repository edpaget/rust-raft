use std::collections::HashMap;
use url::Url;
use std::io::timer::Timer;
use std::time::Duration;

enum State {
    Follower,
    Candidate,
    Leader,
}

struct ClusterState<'a>(HashMap<&'a str,Url>);

struct LeaderState<'a> {
    next_index: HashMap<&'a str,uint>,
    match_index: HashMap<&'a str,uint>,
}

struct ServerState<'a> {
    state: State,
    current_term: uint,
    voted_for: Option<uint>,
    log: Vec<&'a str>,
    commit_index: uint,
    last_applied: uint,
    cluster_state: ClusterState<'a>,
    leader_state: Option<LeaderState<'a>>,
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
