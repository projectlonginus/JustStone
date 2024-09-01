use std::mem::replace;
use crate::{
    structure::{
        utils::{
            enums::{EncryptType, ParseError},
            structs::define::{EncryptionInfo, SecurePacket, StructStone}
        }
    },
    utility::{
        secure::{
            crypto::Crypto,
            utils::AesGcmSivCrypto
        }
    }
};
use crate::structure::utils::enums::EncryptionFlag;

impl SecurePacket {
    pub fn new() -> SecurePacket {
        SecurePacket {
            // encryption_flag:            [0,0,0,0],
            encrypt_data_block_length:  0,
            encrypted_packet:           Default::default(),
            origin_packet:              StructStone::new(),
        }
    }

    pub fn set(&self, encrypt_data_size: usize,mut origin_packet: StructStone) -> SecurePacket {
        Self {
            // encryption_flag:            EncryptionFlag::from_info(encryption_info).to_bytes(),
            encrypt_data_block_length:  encrypt_data_size as u32,
            encrypted_packet:           replace(&mut origin_packet.stone, Default::default()),
            origin_packet,
        }
    }

    pub fn build(mut source: StructStone, encryption_flag: &EncryptionFlag) -> Result<SecurePacket, ParseError> {
        let packet = SecurePacket::new();
        let mut encrypt_method = match encryption_flag.get_encryption_type() {
            EncryptType::AesGcmSiv => AesGcmSivCrypto::default(),
            _ => return Err(ParseError::Unimplemented("EncryptionInfo algorithms other than AesGcmSiv have not yet been implemented.".to_string()))
        };

        source.stone = encrypt_method.encrypt(source.stone);
        Ok(packet.set(source.stone.len(), source))
    }
}