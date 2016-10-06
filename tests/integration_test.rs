extern crate zmq;
extern crate libnm;

use libnm::{Client, TcpTransport};

#[test]
fn test_get_version() {
    let t = TcpTransport::new("localhost", 5555).unwrap();
    let mut client = Client::new(Box::new(t));
    let version = client.get_version().unwrap();
    assert_eq!(version, "0.1.0");
}
