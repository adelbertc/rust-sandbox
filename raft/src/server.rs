use rpc::{Message, RequestVote};
use std::time::{Duration, SystemTime};
use types::{ServerId, Term};

use std::sync::mpsc::{channel, Receiver, Sender};

enum ServerState {
    Leader,
    Follower,
    Candidate,
}

pub struct Server {
    state: ServerState,
    server_id: ServerId,
    current_term: Term,
    voted_for: Option<ServerId>,
    cluster: Vec<Sender<Message>>,
    receive_rpc: Receiver<Message>,
}

impl Server {
    pub fn new(
        server_id: ServerId,
        cluster: Vec<Sender<Message>>,
        receive_rpc: Receiver<Message>,
    ) -> Server {
        Server {
            state: ServerState::Follower,
            server_id: server_id,
            current_term: Term::new(),
            voted_for: None,
            cluster: cluster,
            receive_rpc: receive_rpc,
        }
    }

    pub fn start(&mut self) -> ! {
        let timeout = Duration::from_secs(3);

        let mut received = false;
        let mut last_received = SystemTime::now();
        loop {
            if let Ok(message) = self.receive_rpc.try_recv() {
                received = true;
                last_received = SystemTime::now();
                let Message { sender, message } = message;
                match message {
                    RequestVote::Request {
                        term,
                        candidate_id,
                        last_log_index: _,
                        last_log_term: _,
                    } => if term < self.current_term {
                        let response = RequestVote::Response {
                            term: self.current_term,
                            vote_granted: false,
                        };
                        sender.send(response).unwrap_or(());
                    } else if (self.voted_for.is_none() || self.voted_for == Some(candidate_id))
                        && true
                    /* TODO */
                    {
                        let response = RequestVote::Response {
                            term: self.current_term,
                            vote_granted: true,
                        };
                        sender.send(response).unwrap_or(());
                    } else {
                        let response = RequestVote::Response {
                            term: self.current_term,
                            vote_granted: false,
                        };
                        sender.send(response).unwrap_or(());
                    },
                    RequestVote::Response {
                        term: _,
                        vote_granted: _,
                    } => (),
                }
            }

            if !received && last_received.elapsed().unwrap() > timeout {
                self.begin_election();
            }
        }
    }

    fn begin_election(&mut self) -> () {
        self.state = ServerState::Candidate;
        self.current_term.increment();

        let mut responses = Vec::new();

        let message = RequestVote::Request {
            term: self.current_term,
            candidate_id: self.server_id,
            // TODO
            last_log_index: 0,
            last_log_term: Term::new(),
        };

        for server in self.cluster.iter() {
            let (sender, receiver) = channel();
            responses.push(receiver);

            let message = Message {
                sender,
                message: message.clone(),
            };

            server.send(message).unwrap_or(());
        }
    }
}
