use crate::stprotocol::utils::Cipher;
use crate::structure::utils::structs::define::EncryptionInfo;

pub struct TestSession;

pub struct TestSSL{
    pub encryption: EncryptionInfo,
    pub cipher: Cipher,
}