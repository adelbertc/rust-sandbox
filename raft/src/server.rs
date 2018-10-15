use rpc::{Message, Request, Response, Rpc};
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
    cluster: Vec<Sender<Rpc<Request, Response>>>,
    receive_rpc: Receiver<Rpc<Request, Response>>,
}

impl Server {
    pub fn new(
        server_id: ServerId,
        cluster: Vec<Sender<Rpc<Request, Response>>>,
        receive_rpc: Receiver<Rpc<Request, Response>>,
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
                let Rpc {
                    sender,
                    message: Message { term, payload },
                } = message;

                match payload {
                    Request::RequestVote {
                        candidate_id,
                        last_log_index: _,
                        last_log_term: _,
                    } => {
                        self.current_term.update(&term);

                        if term < self.current_term {
                            let response = Message {
                                term: self.current_term,
                                payload: Response::reject_vote(),
                            };

                            sender.send(response).unwrap_or(());
                        } else if (self.voted_for.is_none() || self.voted_for == Some(candidate_id))
                            && true
                        /* TODO */
                        {
                            let response = Message {
                                term: self.current_term,
                                payload: Response::grant_vote(),
                            };

                            sender.send(response).unwrap_or(());
                        } else {
                            let response = Message {
                                term: self.current_term,
                                payload: Response::reject_vote(),
                            };

                            sender.send(response).unwrap_or(());
                        }
                    }
                    _ => (),
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

        let message = Message {
            term: self.current_term,
            payload: Request::RequestVote {
                candidate_id: self.server_id,
                // TODO
                last_log_index: 0,
                last_log_term: Term::new(),
            },
        };

        for server in self.cluster.iter() {
            let (sender, receiver) = channel();
            responses.push(receiver);

            let message = Rpc {
                sender,
                message: message.clone(),
            };

            server.send(message).unwrap_or(());
        }
    }
}
