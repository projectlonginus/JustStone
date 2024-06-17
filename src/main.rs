use stprotocol::{Client, Handlers};
use structure::enums::EncryptType::NotEncryption;
use crate::structure::enums::EncryptType::AesGcmSiv;
use crate::structure::enums::HandshakeType::RSA;

mod malware;
mod stprotocol;
mod structure;
mod utility;

fn main() {
    let mut client = Client::new("127.0.0.1");
    client.use_encrypt(AesGcmSiv, RSA);

    client.default_client_handler();
}