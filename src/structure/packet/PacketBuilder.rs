use crate::structure::structs::define::{PacketBuilder, StructStone, StructStoneHeader};

impl PacketBuilder {
    pub fn packet(&self) -> StructStone {
        let mut output = self.output();
        StructStone::build(
            StructStoneHeader::build(
                self.is_compression(),
                self.protocol(),
                output.get_size(),
            ),
            output,
        )
    }
}
