use crate::structure::structs::define::StructStoneHeader;

impl StructStoneHeader {
    pub fn from(stone_status: Vec<u8>, stone_type: Vec<u8>, stone_size: Vec<u8>) -> StructStoneHeader {
        StructStoneHeader {
            stone_status,
            stone_type,
            stone_size,
        }
    }

    pub fn new() -> StructStoneHeader {
        StructStoneHeader {
            stone_status: vec![],
            stone_type: vec![],
            stone_size: vec![],
        }
    }

    pub fn set_stone_status(&mut self, stone_status: Vec<u8>) {
        self.stone_status = stone_status;
    }

    pub fn set_stone_type(&mut self, stone_type: Vec<u8>) {
        self.stone_type = stone_type;
    }

    pub fn set_stone_size(&mut self, stone_size: Vec<u8>) {
        self.stone_size = stone_size;
    }

    pub fn take_stone_status(&self) -> &Vec<u8> {
        &self.stone_status
    }

    pub fn take_stone_type(&self) -> &Vec<u8> {
        &self.stone_type
    }

    pub fn take_stone_size(&self) -> &Vec<u8> {
        &self.stone_size
    }
    pub fn is_compression(&self) -> bool {
        match self.stone_status[..] {
            [1, 0, 0, 0] => true,
            _ => false
        }
    }

    pub fn default() -> StructStoneHeader {
        StructStoneHeader {
            stone_status: vec![0, 0, 0, 0],
            stone_type: vec![0, 0, 0, 0],
            stone_size: vec![12, 0, 0, 0],
        }
    }
}