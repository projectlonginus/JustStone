use crate::structure::enums::ParseError;
use crate::structure::structs::define::{SecureHandshakePacket, StructStone};

impl SecureHandshakePacket {
    pub fn new() -> SecureHandshakePacket {
        SecureHandshakePacket {
            encrypt_data_block_length: vec![0, 0, 0, 0, 0, 0, 0, 0],
            handshake_type: vec![0, 0],
            encrypt_type: vec![0, 0, 0, 0],
            encrypted_packet: StructStone::new(),
        }
    }

    pub fn set(&self, encrypt_data_block_length: usize, handshake_type: Vec<u8>, encrypt_type: Vec<u8>, encrypted_packet: StructStone) -> Result<SecureHandshakePacket, ParseError> {
        let mut data_size: Vec<u8> = encrypt_data_block_length.to_le_bytes().to_vec();
        data_size.resize(8, 0);
        if handshake_type.len() != 2 { return Err(ParseError::SizeIsNot2Bytes); }
        if encrypt_type.len() != 4 { return Err(ParseError::SizeIsNot4Bytes); }
        Ok(Self {
            encrypt_data_block_length: data_size,
            handshake_type,
            encrypt_type,
            encrypted_packet,
        })
    }
}