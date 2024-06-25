use crate::structure::utils::{
    structs::define::StructStoneHeader,
    traits::define::ProtocolCodec,
};
use crate::structure::utils::enums::StatusCode;

impl StructStoneHeader {
    pub fn load(packet: Vec<u8>) -> StructStoneHeader {
        let Status =  [packet[0], packet[1], packet[2], packet[3]];
        let Type   =  [packet[4], packet[5], packet[6], packet[7]];
        let mut Size   =  0;
        Size += for n in 8..11 { packet[n] };

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
                true => vec![0, 0, 0, 1],
                false => vec![0, 0, 0, 0], },
            protocol.to_bytes(),
            vec![],
        );
        println!("{:?}", size);
        header.set_stone_size(size);
        header
    }
}