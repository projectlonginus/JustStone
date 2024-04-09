mod malware;
mod stprotocol;
mod structure;

use stprotocol::{Handlers, Client};
use structure::{Detector};

fn main() {
    let mut client = Client::new("127.0.0.1");

    client.default_client_handler()
}