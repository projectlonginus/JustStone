use crate::structure::utils::{
    structs::define::{
        StructStone,
        StructStoneHeader,
        StructStonePayload,
    },
    traits::define::TypeManager,
};

impl StructStone {
    pub fn build(header: StructStoneHeader, payload: StructStonePayload) -> StructStone {
        let mut stone: Vec<u8> = header.to_vec();
        if header.stone_size == 0 {
            return StructStone::from(header, payload, stone);
        }
        if !payload.is_empty() {
            stone.extend(payload.to_vec());
        }
        StructStone::from(header, payload, stone)
    }

    pub fn default() -> StructStone {
        StructStone::build(StructStoneHeader::default(), StructStonePayload::default())
    }

    pub fn buffer() -> StructStone {
        StructStone::build(StructStoneHeader::default(), StructStonePayload::new())
    }
}