use crate::structure::structs::define::SecureHandshakePacket;

impl SecureHandshakePacket {
    pub fn new() -> SecureHandshakePacket {
        SecureHandshakePacket {
            encrypt_data_block_length: vec![],
            handshake_type: vec![],
            encrypt_type: vec![],
            encrypted_packet: vec![],
        }
    }

    pub fn set(encrypt_data_block_length: usize, handshake_type: Vec<u8>, encrypt_type: Vec<u8>, encrypted_packet: Vec<u8>) -> SecureHandshakePacket {
        let mut data_size: Vec<u8> = encrypt_data_block_length.to_le_bytes().to_vec();
        data_size.resize(4, 0);
        SecureHandshakePacket {
            encrypt_data_block_length: data_size,
            handshake_type,
            encrypt_type,
            encrypted_packet,
        }
    }
}