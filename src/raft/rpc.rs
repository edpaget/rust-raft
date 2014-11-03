use capnp::capability::{FromServer, Server};
use capnp_rpc::capabiliyt::{LocalClient};
use capnp_rpc::ex_rpc::EzRPCServer;

use raft_server_capnp::raft_server;

struct RaftServerImpl {
    raft_server: Raft::ServerState,
}

impl raft_server for RaftServerImpl {
    fn append_entries(&mut self, mut context : raft_server::AppendEntriesContext) {
        let (params, results) = context.get();
        match self.raft_server.append_entries(params.get_term(),
                                              params.get_leader_id(),
                                              params.get_prev_log_index(),
                                              params.get_prev_log_term(),
                                              params.get_entries(),
                                              params.get_leader_commit()) {
            Ok((term : u64, success : bool)) => {
                results.set_value(
                    FromServer::new(
                        None::<LocaClient>,
                        box term,
                        box success))
            }
            Err(_) => return context.fail(),
        }

        context.done();
    }
    fn request_vote(&mut self, mut content : raft_server::RequestVoteContext) {
        let (params, results) = context.get();
        match self.raft_server.request_vote(params.get_term(),
                                            params.get_candidated_id(),
                                            params.get_last_log_index(),
                                            params.get_last_log_term()) {
            Ok((term : u64, vote : bool)) => {
                results.set_value(
                    FromServer::new(
                        None::<LocalClient>,
                        box term,
                        box vote))
            }
            Err(_) => return context.fail(),
        }

        context.done();
    }
}

fn start_server(&mut server_state : Raft::ServerState, address : &str) {
    let rpc_server = EzRpcServer::new(address).unwrap();

    let raft_server = (box raft_server::ServerDispatch { server: box RaftServerImpl { raft_server: server_state } }) as Box<Server+Send>;
    rpc_server.export_cap("raft_server", raft_server);

    rpc_server.serve();
}
