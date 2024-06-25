use crate::structure::utils::enums::{StatusCode, StoneTransferProtocol};
use crate::structure::utils::structs::define::{StructStone, StructStoneHeader, StructStonePayload};

impl StructStone {
    pub fn set(&mut self, source: StructStone) {
        self.header = source.header;
        self.payload = source.payload;
        self.stone = source.stone;
    }

    pub fn set_stone(&mut self, source: Vec<u8>) {
        self.stone = source
    }

    pub fn set_header(&mut self, stone_status: StatusCode, stone_type: StoneTransferProtocol, stone_size: usize) {
        self.header.set_stone_status(stone_status);
        self.header.set_stone_type(stone_type);
        self.header.set_stone_size(stone_size);
    }
    pub fn set_payload(&mut self, sys_info: Vec<u8>, command: Vec<u8>, response: Vec<u8>, file: Vec<u8>) {
        self.payload.sysinfo = sys_info;
        self.payload.command_input = command;
        self.payload.response = response;
        self.payload.file = file;
    }
    pub fn from(header: StructStoneHeader, payload: StructStonePayload, stone: Vec<u8>) -> StructStone {
        StructStone {
            header,
            payload,
            stone,
        }
    }
    pub fn new() -> StructStone {
        StructStone {
            header: StructStoneHeader::new(),
            payload: StructStonePayload::new(),
            stone: vec![],
        }
    }
}