extern crate zmq;
extern crate protobuf;

pub mod protocol;

use protocol::messages;
use protobuf::*;

pub struct Connection {
    ctx: zmq::Context,
    socket: zmq::Socket
}

impl Connection {
    pub fn new(addr: &str) -> Connection {
        let mut ctx = zmq::Context::new();

        let mut socket = match ctx.socket(zmq::REQ) {
            Ok(socket) => { socket },
            Err(e) => { panic!(e) }
        };

        match socket.connect(addr) {
            Ok(()) => (),
            Err(e) => panic!(e)
        }

        Connection{
            ctx: ctx,
            socket: socket
        }
    }

    pub fn close(&mut self) {
        let _ = self.socket.close();
    }

    pub fn get_version(&mut self) -> String {
        let mut req = messages::Request::new();
        req.set_r_type(messages::Request_RequestType::GET_VERSION);
        let requests = vec![req];
        let mut message = messages::Requests::new();
        message.set_requests(RepeatedField::from_vec(requests));
        let bytes = message.write_to_bytes().unwrap();
        self.socket.send(&bytes, 0).unwrap();

        let mut msg = zmq::Message::new().unwrap();
        self.socket.recv(&mut msg, 0).unwrap();
        let responses = parse_from_bytes::<messages::Responses>(&msg).unwrap();
        let response = responses.get_responses().iter().as_slice();
        let res = response[0].get_get_version_response();
        return res.get_version().to_string();
    }
}


impl Drop for Connection {
    fn drop(&mut self) {
        self.close();
    }
}

/*
let connection = libnm::Connection.new("localhost:12345");
let backend = connection.new_backend("webgl", "options");
let graph = connection.new_graph();

// how do async actions work with the api
backend.set_camera_standard();
backend.set_camera_matrix(matrix::vec4::new());
backend.set_camera_projection(matrix::vec4::new());

// let rgraph = connection.new_remote_graph("serv.hey.edu");

let cube = Cube.new(10, 10, 10);
let element = Mesh.new(cube, Sound::StandardMaterial);
graph.add_element(element);
graph.set_items
 */
