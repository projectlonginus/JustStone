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
}