use crate::{
    structure::{
        utils::{
            enums::ParseError,
            structs::define::{SecureHandshakePacket, StructStone}
        }
    }
};

impl SecureHandshakePacket {
    pub fn new() -> SecureHandshakePacket {
        SecureHandshakePacket {
            encrypt_data_block_length: vec![0, 0, 0, 0, 0, 0, 0, 0],
            handshake_type: vec![0, 0],
            encrypt_type: vec![0, 0, 0, 0],
            encrypted_packet: Default::default(),
            origin_packet: StructStone::new(),
        }
    }

    pub fn set(&self, encrypt_data_size: usize, handshake_type: Vec<u8>, encrypt_type: Vec<u8>,mut origin_packet: StructStone) -> Result<SecureHandshakePacket, ParseError> {
        if handshake_type.len() != 2 { return Err(ParseError::SizeIsNot2Bytes); }
        if encrypt_type.len() != 4 { return Err(ParseError::SizeIsNot4Bytes); }

        let mut encrypt_data_block_length: Vec<u8> = encrypt_data_size.to_le_bytes().to_vec();
        let encrypted_packet = origin_packet.stone;

        encrypt_data_block_length.resize(8, 0);
        origin_packet.stone = Default::default();

        Ok(Self {
            encrypt_data_block_length,
            handshake_type,
            encrypt_type,
            encrypted_packet,
            origin_packet,
        })
    }
}