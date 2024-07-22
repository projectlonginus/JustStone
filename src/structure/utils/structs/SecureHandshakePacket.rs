#![allow(dead_code)]

use std::mem::replace;

use crate::{
    structure::{
        utils::{
            enums::EncryptionFlag,
            enums::ParseError,
            structs::define::{SecureHandshakePacket, StructStone},
            traits::ProtocolCodec
        }
    }
};

impl SecureHandshakePacket {
    pub fn new() -> SecureHandshakePacket {
        SecureHandshakePacket {
            encryption_flag:            [0,0,0,0],
            encrypt_data_block_length:  0,
            encrypted_packet:           Default::default(),
            origin_packet:              StructStone::new(),
        }
    }

    pub fn set(&self, encrypt_data_size: usize, encryption_flag: &EncryptionFlag, mut origin_packet: StructStone) -> Result<SecureHandshakePacket, ParseError> {
        Ok(Self {
            encryption_flag: encryption_flag.to_bytes(),
            encrypt_data_block_length: encrypt_data_size as u32,
            encrypted_packet: replace(&mut origin_packet.stone, Default::default()),
            origin_packet,
        })
    }

    pub fn set_flag(&mut self, encryption_flag: EncryptionFlag) { self.encryption_flag = encryption_flag.to_bytes(); }

    pub fn set_size(&mut self, size: usize) {
        self.encrypt_data_block_length = size as u32
    }
}