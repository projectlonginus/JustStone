use crate::{
    structure::{
        enums::{
            EncryptType,
            ParseError,
        },
        structs::define::{
            SecurePacket,
            StructStone,
        },
    },
    utility::secure::{
        crypto::Crypto,
        utils::AesGcmSivCrypto,
    },
};

impl SecurePacket {
    pub fn build(mut source: StructStone, encrypt_type: EncryptType) -> Result<SecurePacket, ParseError> {
        let mut packet = SecurePacket::new();
        let mut encrypt_method = match encrypt_type {
            EncryptType::AesGcmSiv => AesGcmSivCrypto::default(),
            _ => return Err(ParseError::Unimplemented("Encryption algorithms other than AesGcmSiv have not yet been implemented.".to_string()))
        };

        encrypt_method.set_plaintext(source.stone);
        encrypt_method.encrypt().expect("aes.encrypt()");
        source.stone = encrypt_method.take_ciphertext().to_owned();
        Ok(packet.set(encrypt_method.take_ciphertext().len(), source))
    }
}