extern crate zmq;
extern crate protobuf;

pub mod protocol;

pub struct Connection {
}

// struct RemoteGraphAddr {
// }

impl Connection {
    pub fn new(addr: &str) {
        let mut ctx = zmq::Context::new();

        let mut socket = match ctx.socket(zmq::REQ) {
            Ok(socket) => { socket },
            Err(e) => { panic!(e) }
        };

        match socket.connect(addr) {
            Ok(()) => (),
            Err(e) => panic!(e)
        }
    }

    // fn get_version() -> String {
    //     let msg = self.create_message(MessageTypes::GetVersion);
    //     match socket.send_str(msg, 0) {
    //         Ok(()) => (),
    //         Err(e) => get_error(e)
    //     }

    //     let msg = GetVersionResponse{};
    //     socket.recv(&mut msg, 0).unwrap();

    //     return msg.version
    // }

    // fn new_graph(name: str) -> Graph {
    //     let msg = CreateGraphMessage{};
    //     self.send_message(msg);
    // }

    // fn send_message(message: Message) {
    //     self.socket.send(message)
    // }

    // fn new_graph_remote(addr: RemoteGraphAddr) -> Graph {
    // }
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
