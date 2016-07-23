extern crate libnm;

use libnm::Client;

fn main() {
    let mut client = Client::new("tcp://localhost:5555");
    let version = client.version().send();
    println!("nmd version: {}", version);
}
