mod exploits;
mod stprotocol;
mod structure;
mod volcano;

use stprotocol::{Client, HandleProtocols};

fn main() {
    let mut client = Client::new("127.0.0.1");

    client.default_protocol_handler().unwrap();
}