use crate::{
    structure::{
        utils::{
            enums::{StatusCode, StoneTransferProtocol},
            structs::define::StructStoneHeader,
            traits::ProtocolCodec
        }
    }
};

impl StructStoneHeader {
    pub fn from(stone_status: [u8; 4], stone_type: [u8; 4], stone_size: u32) -> StructStoneHeader {
        StructStoneHeader {
            stone_status,
            stone_type,
            stone_size,
        }
    }

    pub fn new() -> StructStoneHeader {
        StructStoneHeader {
            stone_status: [0,0,0,0], // from StatusCode
            stone_type:   [0,0,0,0],   // from StoneTransferProtocol
            stone_size:   0,   // from usize
        }
    }

    pub fn set_stone_status(&mut self, stone_status: StatusCode){
        self.stone_status = stone_status.to_bytes()
    }

    pub fn set_stone_type(&mut self, stone_type: StoneTransferProtocol) {
        self.stone_type = stone_type.to_bytes()
    }

    pub fn set_stone_size(&mut self,size: usize) {
        self.stone_size = size as u32
    }

    pub fn is_compression(&self) -> bool {
        match self.stone_status[..] {
            [0, 0, 0, 1] | [0, 0, 1, 1] => true,
            _ => false
        }
    }

    pub fn is_signed(&self) -> bool {
        match self.stone_status[..] {
            [0, 0, 1, 0] | [0, 0, 1, 1 ] => true,
            _ => false
        }
    }

    pub fn default() -> StructStoneHeader {
        StructStoneHeader {
            stone_status: [0, 0, 0, 0],
            stone_type:   [0, 0, 0, 0],
            stone_size:   12,
        }
    }
}