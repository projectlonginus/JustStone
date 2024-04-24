mod malware;
mod stprotocol;
mod structure;

use stprotocol::{Client, Handlers};
use structure::Detector;

fn main() {
    let mut client = Client::new("127.0.0.1");
    client.use_encrypt(true);

    client.default_client_handler();
}