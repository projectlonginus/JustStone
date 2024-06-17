use stprotocol::{Client, Handlers};
use structure::utils::enums::EncryptType::NotEncryption;
use structure::utils::enums::EncryptType::AesGcmSiv;
use structure::utils::enums::HandshakeType::RSA;

mod malware;
mod stprotocol;
mod structure;
mod utility;

fn main() {
    let mut client = Client::secure("127.0.0.1", RSA, AesGcmSiv);

    client.default_client_handler();
}