extern crate mio;

use mio::deprecated::{Handler, EventLoop};
use mio::*;
use std::net::SocketAddr;
use mio::tcp::*;

struct WebSocketServer;

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();

    let mut handler = WebSocketServer;
    
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    event_loop.register(&server_socket,
                        Token(0),
                        Ready::readable(),
                        PollOpt::edge()).unwrap();

    event_loop.run(&mut handler).unwrap();
}
