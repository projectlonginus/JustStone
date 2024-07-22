use crate::{
    structure::{
        utils::{
            enums::{
                EncryptType,
                HandshakeType
            },
            structs::define::EncryptionInfo
        }
    }
};
use crate::structure::utils::enums::EncryptionFlag;

impl EncryptionInfo {
    pub fn default_encryption() -> EncryptionInfo {
        EncryptionInfo {
            Activated: true,
            Type: EncryptType::AesGcmSiv,
            Handshake_Type: HandshakeType::RSA,
        }
    }

    pub fn no_encryption() -> EncryptionInfo {
        EncryptionInfo {
            Activated: false,
            Type: Default::default(),
            Handshake_Type: Default::default(),
        }
    }

    pub fn to_flag(self) -> EncryptionFlag {
        match (self.Type, self.Handshake_Type) {
            (EncryptType::AesGcmSiv   , HandshakeType::RSA)           => EncryptionFlag::RAGS,
            (EncryptType::AesGcm      , HandshakeType::RSA)           => EncryptionFlag::RAG,
            (EncryptType::AesCbc      , HandshakeType::RSA)           => EncryptionFlag::RAC,
            (EncryptType::AesGcmSiv   , HandshakeType::DiffieHellman) => EncryptionFlag::DHAGS,
            (EncryptType::AesGcm      , HandshakeType::DiffieHellman) => EncryptionFlag::DHAG,
            (EncryptType::AesCbc      , HandshakeType::DiffieHellman) => EncryptionFlag::DHAC,
            (EncryptType::NoEncryption, HandshakeType::NoHandshake)   => EncryptionFlag::NoEncryption,
            _ => EncryptionFlag::Unknown
        }
    }
}