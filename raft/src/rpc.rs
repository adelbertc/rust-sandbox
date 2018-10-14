use types::{ServerId, Term};

use std::sync::mpsc::Sender;

pub struct Message {
    pub sender: Sender<RequestVote>,
    pub message: RequestVote,
}

#[derive(Clone, Debug)]
pub enum RequestVote {
    Request {
        term: Term,
        candidate_id: ServerId,
        last_log_index: usize,
        last_log_term: Term,
    },
    Response {
        term: Term,
        vote_granted: bool,
    },
}
