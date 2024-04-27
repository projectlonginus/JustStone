use crate::structure::{
    enums::Packet,
    structs::define::{PacketBuilder, StructStone, StructStoneHeader},
};

impl PacketBuilder {
    pub fn packet(&self) -> Packet {
        let mut output = self.output();
        Packet::from(
            StructStone::build(
                StructStoneHeader::build(
                    self.is_compression(),
                    self.protocol(),
                    output.get_size(),
                ),
                output,
            )
        )
    }
}
