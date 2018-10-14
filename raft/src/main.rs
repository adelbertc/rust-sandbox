mod rpc;
mod server;
mod types;

use server::Server;
use types::ServerId;

use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (send_0, receive_0) = channel();
    let (send_1, receive_1) = channel();
    let (send_2, receive_2) = channel();

    let mut server0 = Server::new(ServerId(0), vec![send_1.clone(), send_2.clone()], receive_0);
    let mut server1 = Server::new(ServerId(1), vec![send_0.clone(), send_2.clone()], receive_1);
    let mut server2 = Server::new(ServerId(2), vec![send_0.clone(), send_1.clone()], receive_2);

    let s0 = thread::spawn(move || {
        server0.start();
    });

    let s1 = thread::spawn(move || {
        server1.start();
    });

    let s2 = thread::spawn(move || {
        server2.start();
    });

    s0.join().and(s1.join()).and(s2.join()).unwrap_or(());
}
