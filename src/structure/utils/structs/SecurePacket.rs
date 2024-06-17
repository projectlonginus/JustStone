use crate::structure::utils::structs::define::{SecurePacket, StructStone};

impl SecurePacket {
    pub fn new() -> SecurePacket {
        SecurePacket {
            encrypt_data_block_length: vec![0, 0, 0, 0, 0, 0],
            encrypted_packet: Default::default(),
            origin_packet: StructStone::new(),
        }
    }

    pub fn set(&self, encrypt_data_size: usize,mut origin_packet: StructStone) -> SecurePacket {
        let mut encrypt_data_block_length: Vec<u8> = encrypt_data_size.to_le_bytes().to_vec();
        let encrypted_packet = origin_packet.stone;

        encrypt_data_block_length.resize(6, 0);
        origin_packet.stone = Default::default();

        Self {
            encrypt_data_block_length,
            encrypted_packet,
            origin_packet,
        }
    }
}