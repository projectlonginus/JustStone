use crate::{
    structure::utils::{
        enums::{
            EncryptType,
            ParseError,
        },
        structs::define::{
            EncryptionInfo,
            SecurePacket,
            StructStone
        }
    },
    utility::secure::{
        crypto::Crypto,
        utils::AesGcmSivCrypto,
    },
};

impl SecurePacket {
    pub fn build(mut source: StructStone, encryption: &EncryptionInfo) -> Result<SecurePacket, ParseError> {
        let packet = SecurePacket::new();
        let mut encrypt_method = match encryption.Type {
            EncryptType::AesGcmSiv => AesGcmSivCrypto::default(),
            _ => return Err(ParseError::Unimplemented("EncryptionInfo algorithms other than AesGcmSiv have not yet been implemented.".to_string()))
        };

        source.stone = encrypt_method.encrypt(source.stone);
        Ok(packet.set(source.stone.len(), source))
    }
}