extern crate libnm;

use libnm::{Client, TcpTransport, pipe, GetVersionResult};

fn main() {
    let t = TcpTransport::new("localhost", 5555).unwrap();
    let mut client = Client::new(Box::new(t));
    let result = pipe()
        .get_version()
        .send(&mut client)
        .unwrap();
    let get_version_result: GetVersionResult = result.get(0);
    println!("nmd version: {}", get_version_result.version);
}
