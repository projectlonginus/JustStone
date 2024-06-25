use std::mem::replace;
use crate::structure::utils::enums::EncryptionFlag;
use crate::structure::utils::structs::define::{EncryptionInfo, SecurePacket, StructStone};
use crate::structure::utils::traits::define::ProtocolCodec;

impl SecurePacket {
    pub fn new() -> SecurePacket {
        SecurePacket {
            encryption_flag:            [0,0,0,0],
            encrypt_data_block_length:  0,
            encrypted_packet:           Default::default(),
            origin_packet:              StructStone::new(),
        }
    }

    pub fn set(&self, encryption_info: &EncryptionInfo, encrypt_data_size: usize,mut origin_packet: StructStone) -> SecurePacket {
        Self {
            encryption_flag:            EncryptionFlag::from_info(encryption_info).to_bytes(),
            encrypt_data_block_length:  encrypt_data_size as u32,
            encrypted_packet:           replace(&mut origin_packet.stone, Default::default()),
            origin_packet,
        }
    }
}