use crate::structure::enums::HeaderError;
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

    pub fn set_stone_status(&mut self, stone_status: Vec<u8>) -> Result<(), HeaderError> {
        match stone_status.len() {
            4 => self.stone_status = stone_status,
            _ => return Err(HeaderError::StatusIsNot4Bytes)
        }
        Ok(())
    }

    pub fn set_stone_type(&mut self, stone_type: Vec<u8>) -> Result<(), HeaderError> {
        match stone_type.len() {
            4 => self.stone_status = stone_type,
            _ => return Err(HeaderError::TypeIsNot4Bytes)
        }
        Ok(())
    }

    pub fn set_stone_size(&mut self, stone_size: Vec<u8>) -> Result<(), HeaderError> {
        match stone_size.len() {
            4 => self.stone_status = stone_size,
            _ => return Err(HeaderError::SizeIsNot4Bytes)
        }
        Ok(())
    }

    pub fn is_compression(&self) -> bool {
        match self.stone_status[..] {
            [0, 0, 0, 1] | [0, 0, 1, 1] => true,
            _ => false
        }
    }

    pub fn is_encrypted(&self) -> bool {
        match self.stone_status[..] {
            [0, 0, 1, 0] | [0, 0, 1, 1 ] => true,
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