extern crate url;

mod raft;

fn main() {
    raft::start_server();
}
