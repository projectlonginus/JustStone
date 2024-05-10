use stprotocol::{Client, Handlers};
use structure::enums::EncryptType::NotEncryption;

mod malware;
mod stprotocol;
mod structure;
mod utility;

fn main() {
    let mut client = Client::new("127.0.0.1");
    client.use_encrypt(NotEncryption);

    client.default_client_handler();
}