extern crate zmq;

use std::result::Result;

use protobuf::{parse_from_bytes, Message};

use protocol::messages;
use transport::{Transport};

pub struct TcpTransport {
    // It turns out the fields are dropped in the order that they
    // are declared. We want the socket to be dropped before the
    // context. Otherwise, the socket drop will hang indefinitely.
    socket: zmq::Socket,
    _ctx: zmq::Context
}

/// TcpTransport sends messages over TCP using ZeroMQ.
impl TcpTransport {
    /// Creates a new `TcpTransport`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let transport = TcpTransport::new("localhost", 12345);
    /// ```
    pub fn new(addr: &str, port: u16) -> Result<TcpTransport, String> {
        let mut ctx = zmq::Context::new();
        let mut socket = match ctx.socket(zmq::REQ) {
            Ok(socket) => socket,
            Err(err) => { return Err(err.to_string()); }
        };
        let s = format!("tcp://{}:{}", addr, port);

        match socket.connect(&s) {
            Ok(result) => result,
            Err(err) => { return Err(err.to_string()); }
        };

        let transport = TcpTransport{
            _ctx: ctx,
            socket: socket
        };

        return Ok(transport);
    }
}

impl Transport for TcpTransport {
    fn send(
        &mut self, requests: &messages::Requests
    ) -> Result<messages::Responses, String> {
        // Send the requests.
        let bytes = match requests.write_to_bytes() {
            Ok(bytes) => bytes,
            Err(err) => { return Err(err.to_string()); }
        };

        match self.socket.send(&bytes, 0) {
            Ok(_) => {},
            Err(err) => { return Err(err.to_string()); }
        };

        // Receive the responses.
        let mut msg = match zmq::Message::new() {
            Ok(msg) => msg,
            Err(err) => { return Err(err.to_string()); }
        };

        match self.socket.recv(&mut msg, 0) {
            Ok(_) => {},
            Err(err) => { return Err(err.to_string()); }
        }

        match parse_from_bytes::<messages::Responses>(&msg) {
            Ok(responses) => { return Ok(responses) },
            Err(err) => { return Err(err.to_string()); }
        };
    }
}
