use types::{ServerId, Term};

use std::sync::mpsc::Sender;

pub struct Rpc<Request, Response> {
    pub sender: Sender<Message<Response>>,
    pub message: Message<Request>,
}

#[derive(Clone)]
pub struct Message<T> {
    pub term: Term,
    pub payload: T,
}

#[derive(Clone)]
pub struct Entry();

#[derive(Clone)]
pub enum Request {
    RequestVote {
        candidate_id: ServerId,
        last_log_index: usize,
        last_log_term: Term,
    },
    AppendEntries {
        leader_id: ServerId,
        prev_log_index: usize,
        prev_log_term: Term,
        entries: Vec<Entry>,
        leader_commit: usize,
    },
}

#[derive(Clone)]
pub enum Response {
    RequestVote { vote_granted: bool },
    AppendEntries { success: bool },
}

impl Response {
    pub fn grant_vote() -> Response {
        Response::RequestVote { vote_granted: true }
    }

    pub fn reject_vote() -> Response {
        Response::RequestVote {
            vote_granted: false,
        }
    }

    pub fn append_success() -> Response {
        Response::AppendEntries { success: true }
    }

    pub fn append_failure() -> Response {
        Response::AppendEntries { success: false }
    }
}
