use crate::structure::structs::define::{SecurePacket, StructStone};

impl SecurePacket {
    pub fn new() -> SecurePacket {
        SecurePacket {
            encrypt_data_block_length: vec![0, 0, 0, 0, 0, 0],
            encrypted_packet: StructStone::new(),
        }
    }

    pub fn set(&self, encrypt_data_block_length: usize, encrypted_packet: StructStone) -> SecurePacket {
        let mut data_size: Vec<u8> = encrypt_data_block_length.to_le_bytes().to_vec();
        data_size.resize(6, 0);
        Self {
            encrypt_data_block_length: data_size,
            encrypted_packet,
        }
    }
}