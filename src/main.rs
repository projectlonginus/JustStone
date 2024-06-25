use stprotocol::{Client, Handlers};
use structure::{
    utils::{
        enums::{
            EncryptType::AesGcmSiv,
            HandshakeType::RSA
        }
    }
};

mod Application;
mod stprotocol;
mod structure;
mod utility;

fn main() {
    let mut client = Client::normal("127.0.0.1").use_encrypt(true, AesGcmSiv, RSA);

    client.default_client_handler();
}