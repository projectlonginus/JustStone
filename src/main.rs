mod stprotocol;

use stprotocol::{Handlers, Client};

fn main() {
    let mut client = Client::new("127.0.0.1");

    client.default_client_handler().unwrap()
}