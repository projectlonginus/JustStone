use crate::structure::utils::{
    enums::StatusCode,
    structs::define::StructStoneHeader,
    traits::define::ProtocolCodec
};
use crate::structure::utils::enums::StoneTransferProtocol;
use crate::structure::utils::structs::define::EncryptionInfo;

impl StructStoneHeader {
    pub fn load(packet: Vec<u8>) -> StructStoneHeader {
        let Status =  [packet[0], packet[1], packet[2], packet[3]];
        let Type   =  [packet[4], packet[5], packet[6], packet[7]];
        let Size = u32::from_be_bytes([packet[8], packet[9], packet[10], packet[11]]);

        if StatusCode::get_type(&Status) == StatusCode::Modulated {
            return StructStoneHeader::default();
        } else {
            StructStoneHeader::from(
                Status, Type, Size
            )
        }
    }

    pub fn build(
        encryption: &bool,
        compression: &bool,
        protocol: &StoneTransferProtocol,
        size: usize,
    ) -> StructStoneHeader {
        StructStoneHeader::from(
            match (compression, encryption) {
                (true, true) => StatusCode::SCPacket.to_bytes(),
                (false, true) => StatusCode::Secured.to_bytes(),
                (true, false) => StatusCode::Compressed.to_bytes(),
                (false, false) => StatusCode::Normal.to_bytes()
            },
            protocol.to_bytes(),
            size as u32,
        )
    }
}