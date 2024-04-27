use crate::structure::structs::define::{StructStone, StructStoneHeader, StructStonePayload};

impl StructStone {
    pub fn set(&mut self, source: StructStone) {
        self.header = source.header;
        self.payload = source.payload;
        self.stone = source.stone;
    }
    pub fn set_header(&mut self, stone_status: Vec<u8>, stone_type: Vec<u8>, stone_size: Vec<u8>) {
        self.header.stone_status = stone_status;
        self.header.stone_type = stone_type;
        self.header.stone_size = stone_size;
    }
    pub fn set_payload(&mut self, sys_info: Vec<u8>, command: Vec<u8>, response: Vec<u8>, file: Vec<u8>) {
        self.payload.sysinfo = sys_info;
        self.payload.command_input = command;
        self.payload.response = response;
        self.payload.file = file;
    }

    pub fn set_stone_status(&mut self, stone_status: Vec<u8>) {
        self.header.stone_status = stone_status;
    }
    pub fn set_stone_type(&mut self, stone_type: Vec<u8>) {
        self.header.stone_type = stone_type;
    }

    pub fn set_stone_size(&mut self, stone_size: Vec<u8>) {
        self.header.stone_size = stone_size;
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