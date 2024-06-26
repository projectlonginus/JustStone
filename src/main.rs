use stprotocol::{Client, Handlers};

mod Application;
mod stprotocol;
mod structure;
mod utility;

fn main() {
    let mut client = Client::secure("127.0.0.1");

    client.default_client_handler();
}