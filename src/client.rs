extern crate zmq;

use protocol::messages;
use protobuf::{parse_from_bytes, RepeatedField, Message};
use types::{Id};
use renderers::{Renderer, RendererOptions, CreateRendererOperation};

pub struct GetVersionOperation<'a> {
    client: &'a mut Client
}

impl<'a> GetVersionOperation<'a> {
    pub fn new(client: &'a mut Client) -> GetVersionOperation<'a> {
        GetVersionOperation{
            client: client
        }
    }

    pub fn send(&mut self) -> String {
        let mut req = messages::Request::new();
        req.set_request_type(messages::Request_RequestType::GET_VERSION);
        let res = self.client.send(req);
        res.get_get_version_response().get_version().to_string()
    }
}

pub struct Client {
    ctx: zmq::Context,
    socket: zmq::Socket
}

impl<'a> Client {
    pub fn new(addr: &str) -> Client {
        let mut ctx = zmq::Context::new();

        let mut socket = match ctx.socket(zmq::REQ) {
            Ok(socket) => { socket },
            Err(e) => panic!(e)
        };

        match socket.connect(addr) {
            Ok(()) => (),
            Err(e) => panic!(e)
        }

        Client{
            ctx: ctx,
            socket: socket
        }
    }

    pub fn close(&mut self) {
        let _ = self.socket.close();
    }

    pub fn send(&mut self, request: messages::Request) -> messages::Response {
        let mut message = messages::Requests::new();
        message.set_requests(RepeatedField::from_vec(vec!(request)));
        let bytes = message.write_to_bytes().unwrap();
        self.socket.send(&bytes, 0).unwrap();

        // Get response.
        let mut msg = zmq::Message::new().unwrap();
        self.socket.recv(&mut msg, 0).unwrap();
        let mut responses = parse_from_bytes::<messages::Responses>(&msg).unwrap();
        responses.take_responses().into_vec().pop().unwrap()
    }

    pub fn get_version(&mut self) -> GetVersionOperation {
        GetVersionOperation::new(self)
    }

    pub fn create_renderer(
        &mut self,
        renderer_options: &'a RendererOptions
    ) -> CreateRendererOperation {
        CreateRendererOperation::new(self, renderer_options)
    }

    pub fn get_renderer_with_id(&mut self, id: Id) -> Renderer {
        Renderer{
            client: self,
            id: id
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.close();
    }
}
