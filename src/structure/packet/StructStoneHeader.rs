use crate::structure::utils::{
    enums::StatusCode,
    structs::define::StructStoneHeader,
    traits::define::ProtocolCodec
};

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
        compression: &bool,
        protocol: &crate::structure::utils::enums::StoneTransferProtocol,
        size: usize,
    ) -> StructStoneHeader {
        let mut header = StructStoneHeader::from(
            match &compression {
                true  => [0, 0, 0, 1],
                false => [0, 0, 0, 0], },
            protocol.to_bytes(),
            0,
        );
        header.set_stone_size(size);
        header
    }
}