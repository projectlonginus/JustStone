use crate::structure::utils::{
    structs::define::StructStoneHeader,
    traits::define::ProtocolCodec,
};

impl StructStoneHeader {
    pub fn load(packet: Vec<u8>) -> StructStoneHeader {
        if packet[0..4] != vec![0, 0, 0, 0] {
            return StructStoneHeader::default();
        } else {
            StructStoneHeader::from(
                Vec::from(&packet[0..4]),
                Vec::from(&packet[4..8]),
                Vec::from(&packet[8..12]),
            )
        }
    }

    pub fn build(
        compression: &bool,
        protocol: &crate::structure::utils::enums::StoneTransferProtocol,
        size: usize,
    ) -> StructStoneHeader {
        let stone_status: Vec<u8> = match &compression {
            true => vec![0, 0, 0, 1],
            false => vec![0, 0, 0, 0],
        };
        let stone_type: Vec<u8> = protocol.to_vec();
        let mut stone_size: Vec<u8> = size.to_le_bytes().to_vec();
        stone_size.resize(4, 0);

        StructStoneHeader::from(
            stone_status,
            stone_type,
            stone_size,
        )
    }
}