use stprotocol::{Client, Handlers};
use structure::Detector;

mod malware;
mod stprotocol;
mod structure;

fn main() {
    let mut client = Client::new("127.0.0.1").use_encrypt(true);

    client.default_client_handler()
}