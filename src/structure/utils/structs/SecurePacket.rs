use crate::structure::structs::define::SecurePacket;

impl SecurePacket {
    pub fn new() -> SecurePacket {
        SecurePacket {
            encrypt_data_block_length: vec![],
            encrypted_packet: vec![],
        }
    }

    pub fn set(encrypt_data_block_length: usize, encrypted_packet: Vec<u8>) -> SecurePacket {
        let mut data_size: Vec<u8> = encrypt_data_block_length.to_le_bytes().to_vec();
        data_size.resize(4, 0);
        SecurePacket {
            encrypt_data_block_length: data_size,
            encrypted_packet,
        }
    }
}