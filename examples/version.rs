extern crate libnm;

use libnm::Connection;

fn main() {
    let mut conn = Connection::new("tcp://localhost:5555");
    let version = conn.get_version();
    println!("{}", version);
}
